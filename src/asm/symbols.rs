#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    symbol_type: Type,
    offset: Option<u32>,
}

impl Symbol {
    pub fn new(name: String, symbol_type: Type) -> Symbol {
        Symbol {
            name,
            symbol_type,
            offset: None,
        }
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offset = Some(offset);
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Label,
    Integer,
    IrString,
}

#[derive(Debug, Clone, Default)]
pub struct Table {
    pub symbols: Vec<Symbol>,
}

impl Table {
    pub fn new() -> Table {
        Table { symbols: vec![] }
    }

    pub fn add(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn has(&self, s: &str) -> bool {
        for symbol in &self.symbols {
            if symbol.name == s {
                return true;
            }
        }
        false
    }

    pub fn value(&self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s {
                return symbol.offset;
            }
        }
        None
    }

    pub fn set_offset(&mut self, s: &str, offset: u32) {
        for symbol in &mut self.symbols {
            if symbol.name == s {
                symbol.offset = Some(offset);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table() {
        let mut sym = Table::new();
        let mut new_sym = Symbol::new("test".to_string(), Type::Label);
        new_sym.set_offset(12);
        sym.add(new_sym);
        assert_eq!(sym.symbols.len(), 1);
        let v = sym.value("test");
        assert!(v.is_some());
        assert_eq!(v.unwrap(), 12);
        let v = sym.value("not_exist");
        assert!(v.is_none());
    }
}
