from pathlib import Path
import sys


def cli_main() -> None:
    from r6502asm.main import main
    result = main(cwd=Path.cwd(), argv=sys.argv[1:])
    if result is None:
        sys.exit(0)
    if isinstance(result, bool):
        sys.exit(0 if result else 1)
    elif isinstance(result, int):
        sys.exit(result)
    else:
        raise NotImplementedError(type(result))


if __name__ == "__main__":
    cli_main()
