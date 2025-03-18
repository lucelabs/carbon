use askama::Template;

#[derive(Debug)]
pub struct DataSourceData {
    pub module_name: String,
}

#[derive(Debug)]
pub struct DecoderData {
    pub module_name: String,
}

#[derive(Template)]
#[template(path = "project.askama", escape = "none", ext = ".askama")]
pub struct ProjectTemplate<'a> {
    pub data_source: &'a DataSourceData,
    pub decoders: &'a [DecoderData],
}
