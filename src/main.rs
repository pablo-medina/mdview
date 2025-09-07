#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
use clap::Parser;
use eframe::egui;
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use sys_locale::get_locale;

#[derive(Parser)]
#[command(name = "mdview")]
#[command(about = "A cross-platform markdown viewer")]
#[command(version = "0.1.0")]
struct Args {
    /// Markdown file to open
    #[arg(help = "Path to the markdown file to view")]
    file: Option<PathBuf>,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
enum Language {
    English,
    Spanish,
    System,
}

impl Default for Language {
    fn default() -> Self {
        Self::System
    }
}

impl Language {
    fn to_string(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Español",
            Language::System => "System / Sistema",
        }
    }

    fn detect_system_language() -> Language {
        if let Some(locale) = get_locale() {
            if locale.starts_with("es") {
                return Language::Spanish;
            }
        }
        Language::English // Default fallback
    }

    fn get_effective_language(&self) -> Language {
        match self {
            Language::System => Self::detect_system_language(),
            lang => lang.clone(),
        }
    }
}

#[derive(Clone)]
struct Translations {
    // Window and app
    app_title: &'static str,
    
    // Toolbar
    open_file: &'static str,
    view_normal: &'static str,
    view_raw: &'static str,
    settings: &'static str,
    
    // Welcome screen
    welcome_title: &'static str,
    welcome_subtitle: &'static str,
    welcome_drag_drop: &'static str,
    open_file_button: &'static str,
    
    // Settings panel
    settings_title: &'static str,
    sections_title: &'static str,
    theme_section: &'static str,
    language_section: &'static str,
    about_section: &'static str,
    apply_button: &'static str,
    close_button: &'static str,
    
    // Theme settings
    theme_config_title: &'static str,
    theme_description: &'static str,
    theme_light: &'static str,
    theme_dark: &'static str,
    theme_system: &'static str,
    preview_label: &'static str,
    
    // Language settings
    language_config_title: &'static str,
    language_description: &'static str,
    language_english: &'static str,
    language_spanish: &'static str,
    language_system: &'static str,
    
    // About section
    about_title: &'static str,
    about_author: &'static str,
    about_year: &'static str,
    about_license: &'static str,
    about_version: &'static str,
    about_technology: &'static str,
    about_description: &'static str,
    about_features: &'static str,
    about_feature_markdown: &'static str,
    about_feature_interface: &'static str,
    about_feature_themes: &'static str,
    about_feature_drag_drop: &'static str,
    about_feature_multiplatform: &'static str,
}

impl Translations {
    fn new(language: &Language) -> Self {
        match language.get_effective_language() {
            Language::Spanish => Self::spanish(),
            _ => Self::english(),
        }
    }

    fn english() -> Self {
        Self {
            app_title: "MDView - Markdown Viewer",
            
            open_file: "Open file",
            view_normal: "Normal view",
            view_raw: "Raw view",
            settings: "Settings",
            
            welcome_title: "MDView - Markdown Viewer",
            welcome_subtitle: "Click 'Open' to load a markdown file",
            welcome_drag_drop: "Or drag a .md file directly here",
            open_file_button: "Open file",
            
            settings_title: "Settings",
            sections_title: "Sections",
            theme_section: "Theme",
            language_section: "Language",
            about_section: "About",
            apply_button: "Apply",
            close_button: "Close",
            
            theme_config_title: "Theme Configuration",
            theme_description: "Select the application theme:",
            theme_light: "Light",
            theme_dark: "Dark",
            theme_system: "System",
            preview_label: "Preview:",
            
            language_config_title: "Language Configuration",
            language_description: "Select the application language:",
            language_english: "English",
            language_spanish: "Spanish",
            language_system: "System",
            
            about_title: "About MDView",
            about_author: "Author: Pablo Medina",
            about_year: "Year: 2025",
            about_license: "License: MIT License",
            about_version: "Version: 0.1.0",
            about_technology: "Technology: Rust + egui",
            about_description: "A cross-platform, modern and efficient Markdown file viewer.",
            about_features: "Features:",
            about_feature_markdown: "Complete support for standard Markdown",
            about_feature_interface: "Modern and responsive interface",
            about_feature_themes: "Light and dark themes",
            about_feature_drag_drop: "Drag and drop files",
            about_feature_multiplatform: "Multiplatform (Windows, Linux, macOS)",
        }
    }

    fn spanish() -> Self {
        Self {
            app_title: "MDView - Visor de Markdown",
            
            open_file: "Abrir archivo",
            view_normal: "Vista normal",
            view_raw: "Vista raw",
            settings: "Configuración",
            
            welcome_title: "MDView - Visor de Markdown",
            welcome_subtitle: "Haz clic en 'Abrir' para cargar un archivo markdown",
            welcome_drag_drop: "O arrastra un archivo .md directamente aquí",
            open_file_button: "Abrir archivo",
            
            settings_title: "Configuración",
            sections_title: "Secciones",
            theme_section: "Tema",
            language_section: "Idioma",
            about_section: "Acerca de",
            apply_button: "Aplicar",
            close_button: "Cerrar",
            
            theme_config_title: "Configuración de Tema",
            theme_description: "Selecciona el tema de la aplicación:",
            theme_light: "Claro",
            theme_dark: "Oscuro",
            theme_system: "Sistema",
            preview_label: "Vista previa:",
            
            language_config_title: "Configuración de Idioma",
            language_description: "Selecciona el idioma de la aplicación:",
            language_english: "Inglés",
            language_spanish: "Español",
            language_system: "Sistema",
            
            about_title: "Acerca de MDView",
            about_author: "Autor: Pablo Medina",
            about_year: "Año: 2025",
            about_license: "Licencia: MIT License",
            about_version: "Versión: 0.1.0",
            about_technology: "Tecnología: Rust + egui",
            about_description: "Un visor de archivos Markdown multiplataforma, moderno y eficiente.",
            about_features: "Características:",
            about_feature_markdown: "Soporte completo para Markdown estándar",
            about_feature_interface: "Interfaz moderna y responsiva",
            about_feature_themes: "Temas claro y oscuro",
            about_feature_drag_drop: "Arrastrar y soltar archivos",
            about_feature_multiplatform: "Multiplataforma (Windows, Linux, macOS)",
        }
    }
}

impl Default for Translations {
    fn default() -> Self {
        Self::english()
    }
}

fn main() -> Result<(), eframe::Error> {
    let args = Args::parse();
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("MDView - Visor de Markdown"),
        ..Default::default()
    };

    eframe::run_native(
        "MDView",
        options,
        Box::new(move |cc| {
            // Configurar fuentes con iconos
            let mut fonts = egui::FontDefinitions::default();
            egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
            cc.egui_ctx.set_fonts(fonts);
            
            // Crear la aplicación con el archivo inicial si se proporcionó
            let mut app = if let Some(storage) = cc.storage {
                MarkdownViewer::from_storage(storage)
            } else {
                MarkdownViewer::default()
            };
            
            // Si se pasó un archivo por línea de comandos, abrirlo
            if let Some(file_path) = args.file {
                app.open_file_from_path(file_path);
            }
            
            Ok(Box::new(app))
        }),
    )
}

#[derive(Serialize, Deserialize)]
struct AppSettings {
    theme: Theme,
    language: Language,
    show_raw_markdown: bool,
    window_maximized: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
enum Theme {
    Light,
    Dark,
    System,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            language: Language::System,
            show_raw_markdown: false,
            window_maximized: false,
        }
    }
}

#[derive(Default)]
struct MarkdownViewer {
    markdown_content: String,
    html_content: String,
    current_file: Option<PathBuf>,
    settings: AppSettings,
    show_settings: bool,
    selected_settings_section: SettingsSection,
    translations: Translations,
}

#[derive(PartialEq)]
enum SettingsSection {
    Theme,
    Language,
    About,
}

impl Default for SettingsSection {
    fn default() -> Self {
        Self::Theme
    }
}

impl MarkdownViewer {
    fn from_storage(storage: &dyn eframe::Storage) -> Self {
        let settings: AppSettings = eframe::get_value(storage, "settings").unwrap_or_default();
        let translations = Translations::new(&settings.language);
        Self {
            settings,
            translations,
            ..Default::default()
        }
    }

    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .pick_file()
        {
            self.open_file_from_path(path);
        }
    }

    fn open_file_from_path(&mut self, path: PathBuf) {
        match fs::read_to_string(&path) {
            Ok(content) => {
                self.markdown_content = content;
                self.current_file = Some(path);
                self.update_html();
            }
            Err(e) => {
                eprintln!("Error al leer el archivo: {}", e);
            }
        }
    }

    fn update_html(&mut self) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let parser = MarkdownParser::new_ext(&self.markdown_content, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        self.html_content = html_output;
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        match self.settings.theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::System => {
                // Usar el tema del sistema (por defecto dark en este caso)
                ctx.set_visuals(egui::Visuals::dark());
            }
        }
    }

    fn update_translations(&mut self) {
        self.translations = Translations::new(&self.settings.language);
    }

    fn render_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(15.0);
            
            // Botón abrir archivo - estilo VSCode
            let open_response = ui.add_sized([40.0, 40.0], 
                egui::Button::new(egui::RichText::new(egui_phosphor::regular::FOLDER_OPEN).size(18.0))
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(egui::CornerRadius::same(6))
            );
            
            if open_response.hovered() {
                ui.painter().rect_filled(
                    open_response.rect, 
                    egui::CornerRadius::same(6), 
                    if ui.visuals().dark_mode { 
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20) 
                    } else { 
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 15) 
                    }
                );
            }
            
            if open_response.on_hover_text(self.translations.open_file).clicked() {
                self.open_file();
            }
            
            ui.add_space(8.0);
            
            // Toggle vista raw - estilo VSCode
            let (icon, tooltip) = if self.settings.show_raw_markdown {
                (egui_phosphor::regular::EYE, self.translations.view_normal)
            } else {
                (egui_phosphor::regular::CODE, self.translations.view_raw)
            };
            
            let view_response = ui.add_sized([40.0, 40.0], 
                egui::Button::new(egui::RichText::new(icon).size(18.0))
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(egui::CornerRadius::same(6))
            );
            
            if view_response.hovered() {
                ui.painter().rect_filled(
                    view_response.rect, 
                    egui::CornerRadius::same(6), 
                    if ui.visuals().dark_mode { 
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20) 
                    } else { 
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 15) 
                    }
                );
            }
            
            if view_response.on_hover_text(tooltip).clicked() {
                self.settings.show_raw_markdown = !self.settings.show_raw_markdown;
            }
            
            ui.add_space(8.0);
            
            // Botón configuración - estilo VSCode
            let settings_response = ui.add_sized([40.0, 40.0], 
                egui::Button::new(egui::RichText::new(egui_phosphor::regular::GEAR).size(18.0))
                    .fill(egui::Color32::TRANSPARENT)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(egui::CornerRadius::same(6))
            );
            
            if settings_response.hovered() {
                ui.painter().rect_filled(
                    settings_response.rect, 
                    egui::CornerRadius::same(6), 
                    if ui.visuals().dark_mode { 
                        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20) 
                    } else { 
                        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 15) 
                    }
                );
            }
            
            if settings_response.on_hover_text(self.translations.settings).clicked() {
                self.show_settings = !self.show_settings;
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(path) = &self.current_file {
                    ui.add_space(15.0);
                    ui.horizontal(|ui| {
                        ui.add(egui::Label::new(egui::RichText::new(egui_phosphor::regular::FILE_TEXT).size(14.0).color(egui::Color32::GRAY)));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new(path.file_name().unwrap_or_default().to_string_lossy()).size(14.0).color(egui::Color32::GRAY));
                    });
                }
            });
        });
    }

    fn render_settings_panel(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }

        egui::Window::new(self.translations.settings_title)
            .default_size([600.0, 400.0])
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Panel izquierdo con secciones
                    ui.vertical(|ui| {
                        ui.set_width(150.0);
                        ui.add_space(10.0);
                        
                        ui.heading(self.translations.sections_title);
                        ui.add_space(15.0);
                        
                        // Botón Tema
                        let theme_selected = self.selected_settings_section == SettingsSection::Theme;
                        let theme_button = egui::Button::new(
                            egui::RichText::new(format!("{} {}", egui_phosphor::regular::PALETTE, self.translations.theme_section)).size(15.0)
                        ).corner_radius(egui::CornerRadius::same(12));
                        
                        let theme_button = if theme_selected {
                            theme_button.fill(if ui.visuals().dark_mode { 
                                egui::Color32::from_rgb(70, 130, 200) 
                            } else { 
                                egui::Color32::from_rgb(90, 150, 220) 
                            })
                        } else {
                            theme_button
                        };
                        
                        if ui.add_sized([150.0, 45.0], theme_button).clicked() {
                            self.selected_settings_section = SettingsSection::Theme;
                        }
                        
                        ui.add_space(8.0);
                        
                        // Botón Idioma
                        let language_selected = self.selected_settings_section == SettingsSection::Language;
                        let language_button = egui::Button::new(
                            egui::RichText::new(format!("{} {}", egui_phosphor::regular::TRANSLATE, self.translations.language_section)).size(15.0)
                        ).corner_radius(egui::CornerRadius::same(12));
                        
                        let language_button = if language_selected {
                            language_button.fill(if ui.visuals().dark_mode { 
                                egui::Color32::from_rgb(70, 130, 200) 
                            } else { 
                                egui::Color32::from_rgb(90, 150, 220) 
                            })
                        } else {
                            language_button
                        };
                        
                        if ui.add_sized([150.0, 45.0], language_button).clicked() {
                            self.selected_settings_section = SettingsSection::Language;
                        }
                        
                        ui.add_space(8.0);
                        
                        // Botón Acerca de
                        let about_selected = self.selected_settings_section == SettingsSection::About;
                        let about_button = egui::Button::new(
                            egui::RichText::new(format!("{} {}", egui_phosphor::regular::INFO, self.translations.about_section)).size(15.0)
                        ).corner_radius(egui::CornerRadius::same(12));
                        
                        let about_button = if about_selected {
                            about_button.fill(if ui.visuals().dark_mode { 
                                egui::Color32::from_rgb(70, 130, 200) 
                            } else { 
                                egui::Color32::from_rgb(90, 150, 220) 
                            })
                        } else {
                            about_button
                        };
                        
                        if ui.add_sized([150.0, 45.0], about_button).clicked() {
                            self.selected_settings_section = SettingsSection::About;
                        }
                    });
                    
                    ui.separator();
                    
                    // Panel derecho con contenido
                    ui.vertical(|ui| {
                        ui.set_min_width(400.0);
                        ui.add_space(10.0);
                        
                        match self.selected_settings_section {
                            SettingsSection::Theme => self.render_theme_settings(ui, ctx),
                            SettingsSection::Language => self.render_language_settings(ui, ctx),
                            SettingsSection::About => self.render_about_section(ui),
                        }
                    });
                });
                
                ui.add_space(10.0);
                ui.separator();
                
                // Botones de acción
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.add_sized([80.0, 35.0], 
                            egui::Button::new(self.translations.close_button)
                                .corner_radius(egui::CornerRadius::same(10))
                        ).clicked() {
                            self.show_settings = false;
                        }
                        ui.add_space(10.0);
                        if ui.add_sized([80.0, 35.0], 
                            egui::Button::new(self.translations.apply_button)
                                .corner_radius(egui::CornerRadius::same(10))
                        ).clicked() {
                            self.apply_theme(ctx);
                        }
                    });
                });
            });
    }

    fn render_theme_settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading(format!("{} {}", egui_phosphor::regular::PALETTE, self.translations.theme_config_title));
        ui.add_space(20.0);
        
        ui.label(self.translations.theme_description);
        ui.add_space(10.0);
        
        // Radio buttons para temas
        ui.vertical(|ui| {
            if ui.radio_value(&mut self.settings.theme, Theme::Light, format!("{} {}", egui_phosphor::regular::SUN, self.translations.theme_light)).clicked() {
                self.apply_theme(ctx);
            }
            ui.add_space(5.0);
            if ui.radio_value(&mut self.settings.theme, Theme::Dark, format!("{} {}", egui_phosphor::regular::MOON, self.translations.theme_dark)).clicked() {
                self.apply_theme(ctx);
            }
            ui.add_space(5.0);
            if ui.radio_value(&mut self.settings.theme, Theme::System, format!("{} {}", egui_phosphor::regular::MONITOR, self.translations.theme_system)).clicked() {
                self.apply_theme(ctx);
            }
        });
        
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Preview del tema
        ui.label(self.translations.preview_label);
        ui.add_space(5.0);
        
        egui::Frame::new()
            .fill(ui.visuals().window_fill())
            .stroke(ui.visuals().window_stroke())
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new("# Ejemplo de Markdown").size(20.0).strong());
                ui.add_space(5.0);
                ui.label("Este es un párrafo de ejemplo con **texto en negrita** y *cursiva*.");
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("•").strong());
                    ui.label("Elemento de lista");
                });
                ui.add_space(5.0);
                ui.label(egui::RichText::new("código inline").monospace().background_color(
                    if ui.visuals().dark_mode { egui::Color32::from_rgb(45, 45, 45) } 
                    else { egui::Color32::from_rgb(240, 240, 240) }
                ));
            });
    }

    fn render_language_settings(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.heading(format!("{} {}", egui_phosphor::regular::TRANSLATE, self.translations.language_config_title));
        ui.add_space(20.0);
        
        ui.label(self.translations.language_description);
        ui.add_space(10.0);
        
        // Radio buttons para idiomas
        ui.vertical(|ui| {
            if ui.radio_value(&mut self.settings.language, Language::English, 
                format!("{} {}", egui_phosphor::regular::GLOBE, self.translations.language_english)).clicked() {
                self.update_translations();
            }
            ui.add_space(5.0);
            if ui.radio_value(&mut self.settings.language, Language::Spanish, 
                format!("{} {}", egui_phosphor::regular::GLOBE, self.translations.language_spanish)).clicked() {
                self.update_translations();
            }
            ui.add_space(5.0);
            if ui.radio_value(&mut self.settings.language, Language::System, 
                format!("{} {}", egui_phosphor::regular::MONITOR, self.translations.language_system)).clicked() {
                self.update_translations();
            }
        });
        
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Vista previa del idioma
        ui.label(self.translations.preview_label);
        ui.add_space(5.0);
        
        egui::Frame::new()
            .fill(ui.visuals().window_fill())
            .stroke(ui.visuals().window_stroke())
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.label(egui::RichText::new(self.translations.welcome_title).size(20.0).strong());
                ui.add_space(5.0);
                ui.label(self.translations.welcome_subtitle);
                ui.add_space(5.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(egui_phosphor::regular::DOT_OUTLINE).strong());
                    ui.label(self.translations.about_feature_interface);
                });
            });
    }

    fn render_about_section(&self, ui: &mut egui::Ui) {
        ui.heading(format!("{} {}", egui_phosphor::regular::INFO, self.translations.about_title));
        ui.add_space(20.0);
        
        // Logo/Icono de la app
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.add(egui::Label::new(egui::RichText::new(egui_phosphor::regular::BOOK_OPEN).size(64.0)));
                ui.add_space(10.0);
                ui.add(egui::Label::new(
                    egui::RichText::new("MDView")
                        .size(28.0)
                        .strong()
                        .color(egui::Color32::from_rgb(51, 102, 153))
                ));
                ui.label(egui::RichText::new(self.translations.app_title).size(14.0).color(egui::Color32::GRAY));
            });
        });
        
        ui.add_space(30.0);
        
        // Información del proyecto
        ui.group(|ui| {
            ui.set_min_width(350.0);
            ui.vertical(|ui| {
                ui.add_space(10.0);
                
                ui.label(format!("{} {}", egui_phosphor::regular::USER, self.translations.about_author));
                ui.add_space(8.0);
                ui.label(format!("{} {}", egui_phosphor::regular::CALENDAR, self.translations.about_year));
                ui.add_space(8.0);
                ui.label(format!("{} {}", egui_phosphor::regular::SCROLL, self.translations.about_license));
                ui.add_space(8.0);
                ui.label(format!("{} {}", egui_phosphor::regular::TAG, self.translations.about_version));
                ui.add_space(8.0);
                ui.label(format!("{} {}", egui_phosphor::regular::WRENCH, self.translations.about_technology));
                
                ui.add_space(10.0);
            });
        });
        
        ui.add_space(20.0);
        
        // Descripción
        ui.label(self.translations.about_description);
        ui.add_space(10.0);
        ui.label(self.translations.about_features);
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.label(format!("{} {}", egui_phosphor::regular::CHECK, self.translations.about_feature_markdown));
                ui.label(format!("{} {}", egui_phosphor::regular::CHECK, self.translations.about_feature_interface));
                ui.label(format!("{} {}", egui_phosphor::regular::CHECK, self.translations.about_feature_themes));
                ui.label(format!("{} {}", egui_phosphor::regular::CHECK, self.translations.about_feature_drag_drop));
                ui.label(format!("{} {}", egui_phosphor::regular::CHECK, self.translations.about_feature_multiplatform));
            });
        });
    }

    fn render_markdown_as_ui(&self, ui: &mut egui::Ui) {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = MarkdownParser::new_ext(&self.markdown_content, options);
        
        let mut current_text = String::new();
        let mut in_code_block = false;
        let mut code_block_content = String::new();
        let mut in_list = false;
        let mut list_item_level = 0;
        let mut in_emphasis = false;
        let mut in_strong = false;
        let mut in_strikethrough = false;
        
        for event in parser {
            match event {
                pulldown_cmark::Event::Start(tag) => {
                    match tag {
                        pulldown_cmark::Tag::Heading(level, _, _) => {
                            ui.add_space(20.0);
                            current_text.clear();
                        }
                        pulldown_cmark::Tag::Paragraph => {
                            if !in_list {
                                ui.add_space(8.0);
                            }
                            current_text.clear();
                        }
                        pulldown_cmark::Tag::CodeBlock(kind) => {
                            ui.add_space(10.0);
                            in_code_block = true;
                            code_block_content.clear();
                            
                            if let pulldown_cmark::CodeBlockKind::Fenced(lang) = kind {
                                if !lang.is_empty() {
                                    ui.add(egui::Label::new(
                                        egui::RichText::new(format!("```{}", lang))
                                            .monospace()
                                            .size(12.0)
                                            .color(egui::Color32::GRAY)
                                    ));
                                }
                            }
                        }
                        pulldown_cmark::Tag::List(_) => {
                            ui.add_space(8.0);
                            in_list = true;
                            list_item_level = 0;
                        }
                        pulldown_cmark::Tag::Item => {
                            current_text.clear();
                        }
                        pulldown_cmark::Tag::Emphasis => {
                            in_emphasis = true;
                        }
                        pulldown_cmark::Tag::Strong => {
                            in_strong = true;
                        }
                        pulldown_cmark::Tag::Strikethrough => {
                            in_strikethrough = true;
                        }
                        pulldown_cmark::Tag::BlockQuote => {
                            ui.add_space(8.0);
                            ui.separator();
                        }
                        _ => {}
                    }
                }
                pulldown_cmark::Event::Text(text) => {
                    if in_code_block {
                        code_block_content.push_str(&text);
                    } else {
                        current_text.push_str(&text);
                    }
                }
                pulldown_cmark::Event::Code(code) => {
                    let mut rich_text = egui::RichText::new(code.as_ref())
                        .monospace()
                        .background_color(egui::Color32::from_rgb(240, 240, 240));
                    
                    if ui.visuals().dark_mode {
                        rich_text = rich_text.background_color(egui::Color32::from_rgb(45, 45, 45));
                    }
                    
                    ui.add(egui::Label::new(rich_text));
                }
                pulldown_cmark::Event::SoftBreak => {
                    current_text.push(' ');
                }
                pulldown_cmark::Event::HardBreak => {
                    current_text.push('\n');
                }
                pulldown_cmark::Event::End(tag) => {
                    match tag {
                        pulldown_cmark::Tag::Heading(level, _, _) => {
                            if !current_text.is_empty() {
                                let mut rich_text = egui::RichText::new(&current_text).strong();
                                
                                match level {
                                    pulldown_cmark::HeadingLevel::H1 => {
                                        rich_text = rich_text.size(28.0).color(egui::Color32::from_rgb(51, 51, 51));
                                    }
                                    pulldown_cmark::HeadingLevel::H2 => {
                                        rich_text = rich_text.size(24.0).color(egui::Color32::from_rgb(68, 68, 68));
                                    }
                                    pulldown_cmark::HeadingLevel::H3 => {
                                        rich_text = rich_text.size(20.0).color(egui::Color32::from_rgb(85, 85, 85));
                                    }
                                    pulldown_cmark::HeadingLevel::H4 => {
                                        rich_text = rich_text.size(18.0).color(egui::Color32::from_rgb(102, 102, 102));
                                    }
                                    pulldown_cmark::HeadingLevel::H5 => {
                                        rich_text = rich_text.size(16.0).color(egui::Color32::from_rgb(119, 119, 119));
                                    }
                                    pulldown_cmark::HeadingLevel::H6 => {
                                        rich_text = rich_text.size(14.0).color(egui::Color32::from_rgb(136, 136, 136));
                                    }
                                }
                                
                                if ui.visuals().dark_mode {
                                    rich_text = match level {
                                        pulldown_cmark::HeadingLevel::H1 => rich_text.color(egui::Color32::WHITE),
                                        pulldown_cmark::HeadingLevel::H2 => rich_text.color(egui::Color32::from_rgb(230, 230, 230)),
                                        pulldown_cmark::HeadingLevel::H3 => rich_text.color(egui::Color32::from_rgb(210, 210, 210)),
                                        _ => rich_text.color(egui::Color32::from_rgb(190, 190, 190)),
                                    };
                                }
                                
                                ui.add(egui::Label::new(rich_text).wrap());
                                
                                if matches!(level, pulldown_cmark::HeadingLevel::H1 | pulldown_cmark::HeadingLevel::H2) {
                                    ui.add_space(5.0);
                                    ui.separator();
                                }
                                ui.add_space(10.0);
                            }
                            current_text.clear();
                        }
                        pulldown_cmark::Tag::Paragraph => {
                            if !current_text.is_empty() {
                                let mut rich_text = egui::RichText::new(&current_text).size(14.0);
                                
                                if in_emphasis {
                                    rich_text = rich_text.italics();
                                }
                                if in_strong {
                                    rich_text = rich_text.strong();
                                }
                                if in_strikethrough {
                                    rich_text = rich_text.strikethrough();
                                }
                                
                                ui.add(egui::Label::new(rich_text).wrap());
                                if !in_list {
                                    ui.add_space(8.0);
                                }
                            }
                            current_text.clear();
                        }
                        pulldown_cmark::Tag::CodeBlock(_) => {
                            if !code_block_content.is_empty() {
                                egui::Frame::new()
                                    .fill(if ui.visuals().dark_mode { 
                                        egui::Color32::from_rgb(30, 30, 30) 
                                    } else { 
                                        egui::Color32::from_rgb(248, 248, 248) 
                                    })
                                    .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200)))
                                    .inner_margin(10.0)
                                    .show(ui, |ui| {
                                        ui.add(
                                            egui::TextEdit::multiline(&mut code_block_content.as_str())
                                                .font(egui::TextStyle::Monospace)
                                                .desired_width(f32::INFINITY)
                                                .desired_rows(code_block_content.lines().count().max(1))
                                        );
                                    });
                                ui.add_space(10.0);
                            }
                            in_code_block = false;
                            code_block_content.clear();
                        }
                        pulldown_cmark::Tag::List(_) => {
                            in_list = false;
                            ui.add_space(8.0);
                        }
                        pulldown_cmark::Tag::Item => {
                            if !current_text.is_empty() {
                                ui.horizontal(|ui| {
                                    ui.add_space(list_item_level as f32 * 20.0);
                                    ui.add(egui::Label::new(egui_phosphor::regular::DOT_OUTLINE));
                                    ui.add_space(5.0);
                                    ui.add(egui::Label::new(&current_text).wrap());
                                });
                            }
                            current_text.clear();
                        }
                        pulldown_cmark::Tag::Emphasis => {
                            in_emphasis = false;
                        }
                        pulldown_cmark::Tag::Strong => {
                            in_strong = false;
                        }
                        pulldown_cmark::Tag::Strikethrough => {
                            in_strikethrough = false;
                        }
                        pulldown_cmark::Tag::BlockQuote => {
                            ui.add_space(8.0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

impl eframe::App for MarkdownViewer {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "settings", &self.settings);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Aplicar tema
        self.apply_theme(ctx);

        // Toolbar superior
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            ui.add_space(5.0);
            self.render_toolbar(ui);
            ui.add_space(5.0);
        });

        // Panel de configuración
        self.render_settings_panel(ctx);

        // Contenido principal
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.markdown_content.is_empty() {
                let full_width = ui.available_width();
                let avail_height = ui.available_height();
                let top_space = (avail_height * 0.18).max(60.0).min(260.0);
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(full_width, 0.0),
                    egui::Layout::top_down(egui::Align::Center),
                    |ui| {
                        ui.add_space(top_space);
                        ui.centered_and_justified(|ui| {
                            let title_text = format!("{} {}", egui_phosphor::regular::BOOK_OPEN, self.translations.welcome_title);
                            ui.add(egui::Label::new(
                                egui::RichText::new(title_text)
                                    .size(32.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(51, 102, 153))
                            ));
                        });
                        ui.add_space(30.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new(self.translations.welcome_subtitle)
                                .size(16.0)
                                .color(egui::Color32::GRAY)
                        ));
                        ui.add_space(20.0);
                        ui.add(egui::Label::new(
                            egui::RichText::new(self.translations.welcome_drag_drop)
                                .size(14.0)
                                .italics()
                                .color(egui::Color32::GRAY)
                        ));
                        ui.add_space(30.0);
                        if ui.add_sized([160.0, 45.0], 
                            egui::Button::new(
                                egui::RichText::new(format!("{} {}", egui_phosphor::regular::FOLDER_OPEN, self.translations.open_file_button))
                                    .size(16.0)
                            ).corner_radius(egui::CornerRadius::same(12))
                        ).clicked() {
                            self.open_file();
                        }
                    }
                );
            } else {
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let max_width = ui.available_width().min(800.0);
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(max_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.add_space(20.0);
                                
                                if self.settings.show_raw_markdown {
                                    egui::Frame::new()
                                        .fill(if ui.visuals().dark_mode { 
                                            egui::Color32::from_rgb(25, 25, 25) 
                                        } else { 
                                            egui::Color32::from_rgb(252, 252, 252) 
                                        })
                                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 200)))
                                        .inner_margin(15.0)
                                        .show(ui, |ui| {
                                            ui.add(
                                                egui::TextEdit::multiline(&mut self.markdown_content.as_str())
                                                    .font(egui::TextStyle::Monospace)
                                                    .desired_width(f32::INFINITY)
                                            );
                                        });
                                } else {
                                    ui.horizontal(|ui| {
                                        ui.add_space(20.0);
                                        ui.vertical(|ui| {
                                            self.render_markdown_as_ui(ui);
                                            ui.add_space(50.0);
                                        });
                                        ui.add_space(20.0);
                                    });
                                }
                            }
                        );
                    });
            }
        });

        // Drag and drop
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for file in &i.raw.dropped_files {
                    if let Some(path) = &file.path {
                        if path.extension().map_or(false, |ext| {
                            matches!(ext.to_str(), Some("md") | Some("markdown") | Some("txt"))
                        }) {
                            self.open_file_from_path(path.clone());
                        }
                    }
                }
            }
        });
    }
}