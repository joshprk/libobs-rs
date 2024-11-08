use syn::{Attribute, Expr};

pub fn collect_doc(attrs: &Vec<Attribute>) -> (Vec<String>, Vec<&Attribute>) {
    let mut docs_str = Vec::new();
    let mut docs_attr = Vec::new();
    for attr in attrs {
        let name_val = match &attr.meta {
            syn::Meta::NameValue(n) => n,
            _ => continue,
        };

        let is_doc = name_val.path.is_ident("doc");
        if !is_doc {
            continue;
        }

        let lit = match &name_val.value {
            Expr::Lit(l) => match &l.lit {
                syn::Lit::Str(s) => s.value(),
                _ => continue,
            },
            _ => continue,
        };

        docs_str.push(lit);
        docs_attr.push(attr);
    }

    (docs_str, docs_attr)
}
