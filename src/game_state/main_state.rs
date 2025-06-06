use crate::game_state::gui_component::*;
use crate::Vector2f;
use crate::GlyphCache;
use crate::game_state::GameState;
use crate::game_state::*;
use crate::Texture;
use piston_window::*;
use super::gui::GUI;


pub struct MainState {
    main_menu: GUI,
    settings_menu: GUI,
    open_settings: bool,
}

impl GameState for MainState {
    fn draw(&self, _: &Game, glyphs: &mut GlyphCache<'static, (), Texture>, c: Context, gl: &mut GlGraphics) {
        if self.open_settings {
            self.settings_menu.draw(glyphs, c, gl);
        } else {
            self.main_menu.draw(glyphs, c, gl);
        }
    }

    fn update(&mut self, control_args: &ControlArgs, e: &Event, game: &mut Game) -> Option<Box<dyn GameState>>{
        let mut next_state = None;
        if self.open_settings {
            for component in self.settings_menu.components.as_mut_slice() {
                match component.update(control_args.cursor_pos(), e, game) {
                    GUIEvent::Custom(event) => {
                        if event.as_str() == "back" {
                            self.open_settings = false;
                        }
                    }
                    _ => {}
                }
            }
        } else {
            for component in self.main_menu.components.as_mut_slice() {
                match component.update(control_args.cursor_pos(), e, game) {
                    GUIEvent::StateChange(state) => next_state = Some(state),
                    GUIEvent::Custom(event) => {
                        if event.as_str() == "open_settings" {
                            self.open_settings = true;
                        }
                    }
                    GUIEvent::Quit => std::process::exit(0),
                    _ => {},
                }
            }
        }
        
        if let Some(Button::Keyboard(Key::Escape)) = e.press_args() {
            if self.open_settings {
                self.open_settings = false;
            } else {
                std::process::exit(0);
            }
        }

        next_state
    }
}

impl From<&Game> for MainState {
    fn from(value: &Game) -> Self {  
        let text = Text::new_color(color::BLACK, 20);
        let mut rect = Rectangle::new_round_border(color::BLACK, 15.0, 1.0).color(color::CYAN);
        let mut button_position = Vector2f::new(540.0, 100.0);
        let button_size = Vector2f::new(200.0, 50.0);

        let display = Display::new(rect, DisplayContent::Text(text, "Sandbox".to_string()));
        let button1 = GUIButton::new(
            button_position, 
            button_size, 
            display,
            |btn, event, game| { 
                match event {
                    GUIEvent::Click => return GUIEvent::StateChange(Box::new(playing_state::PlayingState::from(&*game))),
                    GUIEvent::Hover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 2.0).border,
                    GUIEvent::UnHover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 1.0).border,
                    _ => {}
                }
                event                
            },
        );

        button_position.y += 75.0;
        let display = Display::new(rect, DisplayContent::Text(text, "Multiplayer".to_string()));
        let button2 = GUIButton::new(
            button_position, 
            button_size, 
            display,
            |btn, event, _| { 
                match event {
                    GUIEvent::Click => println!("Multiplayer not implemented yet"),
                    GUIEvent::Hover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 2.0).border,
                    GUIEvent::UnHover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 1.0).border,
                    _ => {}
                }
                event                
            },
        );

        button_position.y += 75.0;
        let display = Display::new(rect, DisplayContent::Text(text, "Settings".to_string()));
        let button3 = GUIButton::new(
            button_position, 
            button_size, 
            display,
            |btn, event, _| { 
                match event {
                    GUIEvent::Click => return GUIEvent::Custom("open_settings".to_string()),
                    GUIEvent::Hover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 2.0).border,
                    GUIEvent::UnHover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 1.0).border,
                    _ => {}
                }
                event
            },
        );

        button_position.y += 75.0;
        rect.color = color::RED;
        let display = Display::new(rect, DisplayContent::Text(text, "Quit".to_string()));
        let button4 = GUIButton::new(
            button_position, 
            button_size, 
            display,
            |btn, event, _| {
                match event {
                    GUIEvent::Click => return GUIEvent::Quit,
                    GUIEvent::Hover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 2.0).border,
                    GUIEvent::UnHover => btn.display.rect.border = Rectangle::new_round_border(color::BLACK, 15.0, 1.0).border,
                    _ => {}
                }
                event
            }, 
        );

        Self { 
            main_menu: GUI { components: vec![Box::new(button1), Box::new(button2), Box::new(button3), Box::new(button4)] },
            settings_menu: GUI::get_settings_menu(value),
            open_settings: false,
        }      
    }
}