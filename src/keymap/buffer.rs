use crossterm::style::{Color, Stylize};

#[derive(Debug)]
pub struct Buffer {
    b: Vec<Vec<char>>,
    c: Vec<Vec<Option<(u8, u8, u8)>>>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            b: Vec::<Vec<char>>::new(),
            c: Vec::<Vec<Option<(u8, u8, u8)>>>::new(),
        }
    }

    pub fn put(&mut self, x: usize, y: usize, c: char, color: &Option<(u8, u8, u8)>) {
        while self.b.len() < y + 1 {
            let v = Vec::<char>::new();
            let vc = Vec::<Option<(u8, u8, u8)>>::new();
            self.b.push(v);
            self.c.push(vc);
        }
        if self.b[y].len() < x + 1 {
            self.b[y].resize(x + 1, ' ');
            self.c[y].resize(x + 1, None);
        }
        self.b[y][x] = c;
        self.c[y][x] = *color;
    }

    pub fn dump(&self) {
        // cut top lines containing only spaces
        let mut spaces_only = true;
        let mut o = String::new();
        let mut last_color: Option<(u8, u8, u8)> = None;
        for (i, line) in self.b.iter().enumerate() {
            if spaces_only {
                for c in line {
                    if *c != ' ' {
                        spaces_only = false;
                        break;
                    }
                }
            }
            if !spaces_only {
                let line_colors = &self.c[i];
                let mut a = String::new();
                for (j, c) in line.iter().enumerate() {
                    let current_color = &line_colors[j];
                    if last_color != *current_color {
                        if let Some((r, g, b)) = last_color {
                            let styled = a.to_owned().with(Color::Black).on(Color::Rgb { r, g, b });
                            o.push_str(&format!("{}", styled).to_owned());
                        } else {
                            o.push_str(&a);
                        }
                        a.truncate(0);
                        last_color = *current_color;
                    }
                    a.push(*c);
                }
                if let Some((r, g, b)) = last_color {
                    let styled = a.to_owned().with(Color::Black).on(Color::Rgb { r, g, b });
                    o.push_str(&format!("{}", styled).to_owned());
                } else {
                    o.push_str(&a);
                }
                last_color = None;
                o.push('\n');
            }
        }
        println!("{}", o);
    }
}
