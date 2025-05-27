# pyright: reportMissingTypeStubs=false

from argparse import ArgumentParser
from dataclasses import dataclass
from pathlib import Path
from tempfile import NamedTemporaryFile
import Ophis
import json
import re
import shutil


ORG_REGEX = re.compile("^\\s*\\.org\\s+(?P<addr>\\$?.+)\\s*$")
MAGIC_NUMBER = 0x6502
DEFAULT_ASM_EXT: str = ".asm"
DEFAULT_EXT: str = ".r6502"
DEFAULT_SYMBOL_EXT: str = ".r6502.json"
ORIGIN_SYMBOL: str = "origin"
START_SYMBOL: str = "start"


@dataclass(frozen=True)
class SymbolInfo:
    name: str
    value: int
    source_location: str | None

    @staticmethod
    def read(map_path: Path) -> list["SymbolInfo"]:
        results: list[SymbolInfo] = []
        with map_path.open("rt") as map_f:
            for line in map_f.readlines():
                parts = [
                    p.strip()
                    for p in line.strip().split("|", maxsplit=2)
                ]
                if len(parts) != 3:
                    raise RuntimeError(f"Syntax error in {map_path}")
                value_str, n, source_location = parts
                value = parse_int(value_str)
                if value is not None:
                    results.append(
                        SymbolInfo(
                            name=n,
                            value=value,
                            source_location=source_location))
        return results


def parse_int(s: str) -> int | None:
    parts = s.split("$", maxsplit=1)
    match len(parts):
        case 2: return int(parts[1], base=16)
        case 1: return int(parts[0])
        case _: return None


def get_symbol(map_path: Path, name: str) -> int | None:
    with map_path.open("rt") as map_f:
        for line in map_f.readlines():
            parts = [
                p.strip()
                for p in line.strip().split("|", maxsplit=2)
            ]
            if len(parts) != 3:
                raise RuntimeError(f"Syntax error in {map_path}")
            addr_s, n, _ = parts
            if n == name:
                return parse_int(addr_s)
    return None


def get_origin(asm_path: Path, default: int) -> int:
    with asm_path.open("rt") as f:
        while True:
            line = f.readline()
            if len(line) == 0:
                break
            m = ORG_REGEX.match(line.strip())
            if m is not None:
                addr_str = m.group("addr")
                assert isinstance(addr_str, str)
                addr = parse_int(addr_str.strip())
                if addr is not None:
                    return addr
    return default


def get_start(map_path: Path, default: int) -> int:
    start = get_symbol(map_path, "start")
    return default if start is None else start


def make_image_path(asm_path: Path, image_path: Path | None) -> Path:
    if image_path is not None:
        return image_path
    d = asm_path.parent
    stem = asm_path.stem
    ext = asm_path.suffix
    if ext.lower() == DEFAULT_ASM_EXT:
        return d / f"{stem}{DEFAULT_EXT}"
    return d / f"{stem}{ext}{DEFAULT_EXT}"


def make_symbol_path(image_path: Path) -> Path:
    d = image_path.parent
    stem = image_path.stem
    ext = image_path.suffix
    if ext.lower() == DEFAULT_EXT:
        return d / f"{stem}{DEFAULT_SYMBOL_EXT}"
    return d / f"{stem}{ext}{DEFAULT_SYMBOL_EXT}"


def assemble(asm_path: Path, image_path: Path | None) -> None:
    image_path = make_image_path(asm_path, image_path)
    origin = get_origin(asm_path, 0x0000)

    with NamedTemporaryFile(delete=True, delete_on_close=False) as bin_temp, NamedTemporaryFile(delete=True, delete_on_close=False) as map_temp:
        bin_path = Path(bin_temp.name)
        map_path = Path(map_temp.name)
        # autopep8: off
        result = Ophis.Ophis.Main.run_ophis([str(asm_path), "--quiet", "-o", str(bin_path), "-m", str(map_path)]) # pyright: ignore[reportUnknownMemberType]
        # autopep8: on
        if result != 0:
            raise RuntimeError("Ophis failed")
        bin_temp.close()

        symbols = SymbolInfo.read(map_path)
        symbol_map = {
            s.name: s
            for s in symbols
        }

        start_symbol = symbol_map.get(START_SYMBOL)
        start = origin if start_symbol is None else start_symbol.value

        if ORIGIN_SYMBOL not in symbol_map:
            symbol_map[ORIGIN_SYMBOL] = SymbolInfo(
                name=ORIGIN_SYMBOL,
                value=origin,
                source_location=None)

        if START_SYMBOL not in symbol_map:
            symbol_map[START_SYMBOL] = SymbolInfo(
                name=START_SYMBOL,
                value=start,
                source_location=None)

        with image_path.open("wb") as image_f:
            _ = image_f.write(MAGIC_NUMBER.to_bytes(2, byteorder="little"))
            _ = image_f.write(origin.to_bytes(2, byteorder="little"))
            _ = image_f.write(start.to_bytes(2, byteorder="little"))
            with bin_path.open("rb") as other_f:
                shutil.copyfileobj(other_f, image_f)

        with make_symbol_path(image_path).open("wt") as symbol_f:
            def transform(symbol: SymbolInfo) -> dict[str, str]:
                d = {
                    "name": symbol.name,
                    "value": f"${symbol.value:04X}"
                }
                if symbol.source_location is not None:
                    d["sourceLocation"] = symbol.source_location
                return d
            json.dump([
                transform(s)
                for s in sorted(symbol_map.values(), key=lambda s: s.name)
            ], symbol_f, indent=2)


def main(cwd: Path, argv: list[str]) -> None:
    def resolved_path(s: str) -> Path:
        return (cwd / Path(s).expanduser()).resolve()

    parser = ArgumentParser(prog="r6502asm")
    _ = parser.add_argument(
        "asm_path",
        metavar="ASM_PATH",
        type=resolved_path,
        help="path to input .asm file")
    _ = parser.add_argument(
        "--output",
        "-o",
        dest="image_path",
        metavar="IMAGE_PATH",
        type=resolved_path,
        default=None,
        help="path to output .r6502 image file")

    args = parser.parse_args(argv)
    assemble(asm_path=args.asm_path, image_path=args.image_path)
