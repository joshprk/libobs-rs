use syn::{
    parse::{Parse, ParseStream},
    Ident, LitStr, Result, Token,
};

pub struct UpdaterInput {
    pub name: LitStr,
    pub updatable_type: Ident,
}

impl Parse for UpdaterInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let n = input.parse()?;
        input.parse::<Token![,]>()?;
        Ok(UpdaterInput {
            name: n,
            updatable_type: input.parse()?,
        })
    }
}
