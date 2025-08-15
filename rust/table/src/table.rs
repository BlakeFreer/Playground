#![allow(dead_code)]

use eframe::egui;

/// T should usually be a reference type.
pub struct Table<'a, T> {
    id: egui::Id,
    rows: Vec<Row<'a, T>>,
    entries: Vec<T>,
    // Configuration
    transposed: bool,
}

type RowFunc<'a, T> = dyn for<'b> Fn(&'b mut T, &mut egui::Ui) -> () + 'a;

impl<'a, T> Table<'a, T> {
    pub fn new(id: &str) -> Self {
        Self {
            id: egui::Id::new(id),
            rows: Vec::new(),
            entries: Vec::new(),
            transposed: false,
        }
    }

    pub fn transpose(mut self, transposed: bool) -> Self {
        self.transposed = transposed;
        self
    }

    pub fn with_entries(mut self, entries: impl Iterator<Item = T>) -> Self {
        self.entries.extend(entries);
        self
    }

    pub fn row<S, F>(&mut self, label: S, accessor: F)
    where
        S: Into<String>,
        F: for<'b> Fn(&'b mut T) -> String + 'static,
    {
        self.rows.push(Row {
            label: label.into(),
            drawer: Box::new(move |t, ui| {
                ui.label(accessor(t));
            }),
        });
    }

    pub fn row_ui<S, F, R>(&mut self, label: S, function: F)
    where
        S: Into<String>,
        F: for<'b> Fn(&'b mut T, &mut egui::Ui) -> R + 'a,
    {
        self.rows.push(Row {
            label: label.into(),
            drawer: Box::new(move |t, ui| drop(function(t, ui))),
        });
    }

    pub fn col(&mut self, data: T) {
        self.entries.push(data);
    }
}

impl<T> egui::Widget for Table<'_, T> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        egui::Grid::new(self.id)
            .striped(true)
            .show(ui, |ui| {
                if self.transposed {
                    for row in &self.rows {
                        ui.label(row.label.clone());
                    }
                    ui.end_row();
                    for entry in &mut self.entries {
                        for row in &self.rows {
                            ui.vertical(|ui| {
                                (row.drawer)(entry, ui);
                            });
                        }
                        ui.end_row();
                    }
                } else {
                    for row in self.rows {
                        ui.label(row.label.clone());
                        for entry in &mut self.entries {
                            ui.vertical(|ui| {
                                (row.drawer)(entry, ui);
                            });
                        }
                        ui.end_row();
                    }
                }
            })
            .response
    }
}

struct Row<'a, T> {
    label: String,
    drawer: Box<RowFunc<'a, T>>,
}
