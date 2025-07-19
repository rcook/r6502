use r6502cpu::symbols::MapFile;
use std::sync::{Arc, Mutex};

pub enum ExportSortOrder {
    ByName,
    ByValue,
}

pub struct ExportListInfo {
    current_sort_order: Arc<Mutex<ExportSortOrder>>,
    by_name: String,
    by_value: String,
}

impl ExportListInfo {
    pub fn new(map_file: &MapFile) -> Self {
        Self {
            current_sort_order: Arc::new(Mutex::new(ExportSortOrder::ByValue)),
            by_name: Self::format_exports(map_file, &ExportSortOrder::ByName),
            by_value: Self::format_exports(map_file, &ExportSortOrder::ByValue),
        }
    }

    pub fn toggle(&self) -> &str {
        let mut current_sort_order = self.current_sort_order.lock().unwrap();
        match *current_sort_order {
            ExportSortOrder::ByName => {
                *current_sort_order = ExportSortOrder::ByValue;
                &self.by_value
            }
            ExportSortOrder::ByValue => {
                *current_sort_order = ExportSortOrder::ByName;
                &self.by_name
            }
        }
    }

    fn format_exports(map_file: &MapFile, sort_order: &ExportSortOrder) -> String {
        let mut exports = map_file.exports.iter().collect::<Vec<_>>();
        match sort_order {
            ExportSortOrder::ByName => exports.sort_by(|a, b| a.name.cmp(&b.name)),
            ExportSortOrder::ByValue => exports.sort_by(|a, b| a.value.cmp(&b.value)),
        }

        exports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    }
}
