use crossterm::style::{Color, Stylize};

#[derive(Debug, Clone)]
struct Position {
    sym: char,
    color: Option<(u8, u8, u8)>,
}

#[derive(Debug)]
pub struct Buffer {
    b: Vec<Vec<Position>>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            b: Vec::<Vec<Position>>::new(),
        }
    }

    pub fn put(&mut self, x: usize, y: usize, c: char, color: &Option<(u8, u8, u8)>) {
        while self.b.len() < y + 1 {
            let v = Vec::<Position>::new();
            self.b.push(v);
        }
        if self.b[y].len() < x + 1 {
            self.b[y].resize(
                x + 1,
                Position {
                    sym: ' ',
                    color: None,
                },
            );
        }
        self.b[y][x] = Position {
            sym: c,
            color: *color,
        };
    }

    pub fn dump(&self) {
        // cut top lines containing only spaces
        let mut spaces_only = true;
        let mut result = String::new();
        let mut last_color: Option<(u8, u8, u8)> = None;
        for line in self.b.iter() {
            if spaces_only {
                for p in line {
                    if p.sym != ' ' {
                        spaces_only = false;
                        break;
                    }
                }
            }
            if !spaces_only {
                let mut colored_substring = String::new();
                for p in line.iter() {
                    if last_color != p.color {
                        if let Some((r, g, b)) = last_color {
                            let styled = colored_substring
                                .to_owned()
                                .with(Color::Black)
                                .on(Color::Rgb { r, g, b });
                            result.push_str(&format!("{}", styled).to_owned());
                        } else {
                            result.push_str(&colored_substring);
                        }
                        colored_substring.truncate(0);
                        last_color = p.color;
                    }
                    colored_substring.push(p.sym);
                }
                if let Some((r, g, b)) = last_color {
                    let styled = colored_substring
                        .to_owned()
                        .with(Color::Black)
                        .on(Color::Rgb { r, g, b });
                    result.push_str(&format!("{}", styled).to_owned());
                } else {
                    result.push_str(&colored_substring);
                }
                last_color = None;
                result.push('\n');
            }
        }
        print!("{}", result);
    }
}
