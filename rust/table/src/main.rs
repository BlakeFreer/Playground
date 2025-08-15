use eframe::{App, egui};
use table::table::Table;

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: impl Into<String>, age: u32) -> Self {
        Self {
            name: name.into(),
            age,
        }
    }
}

struct MyApp {
    people: Vec<Person>,
}

impl MyApp {
    fn draw(&mut self, ui: &mut egui::Ui) {
        ui.heading("Let's show some shit");

        let mut t = Table::new("observer").with_entries(self.people.iter());

        t.row("Name", |p| p.name.clone());
        t.row_ui("Age", |p, ui| ui.heading(p.age.to_string()));

        ui.add(t);

        ui.heading("Let's break some shit");

        let mut t = Table::new("mutator")
            .with_entries(self.people.iter_mut())
            .transpose(true);

        t.row_ui("Name", |p, ui| {
            ui.add(egui::TextEdit::singleline(&mut p.name))
        });
        t.row_ui("Age", |p, ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("+").clicked() {
                    p.age += 1;
                }
                if ui.button("-").clicked() {
                    p.age -= 1;
                }
            });
        });

        ui.add(t);

        ui.heading("Let's count some shit");
        let mut t = Table::new("enumerator").with_entries(self.people.iter().enumerate());

        t.row("ID", |(idx, p)| format!("{} is number {idx}", p.name));

        ui.add(t);
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My beautiful app");
            self.draw(ui);
        });
    }
}

fn main() -> eframe::Result {
    let app = MyApp {
        people: vec![
            Person::new("Bob", 34),
            Person::new("Alice", 39),
            Person::new("Charlie", 30),
        ],
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native("My App", options, Box::new(|_cc| Ok(Box::new(app))))
}
