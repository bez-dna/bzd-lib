// sdk https://github.com/cloudevents/sdk-rust не поддерживает прото, только JSON,
// поэтому я вытащил прото файл, но без билдера, оставим это на потом

use strum_macros::EnumString;

tonic::include_proto!("io.cloudevents.v1");

#[derive(PartialEq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Type {
    Created,
    Updated,
    Deleted,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::cloudevents::CloudEvent;

    #[test]
    fn build_ce_event() {
        let ce = CloudEvent {
            id: "CE_ID".into(),
            source: "SOURCE".into(),
            spec_version: "1.0".into(),
            r#type: "app.bezdna.message.created".into(),
            attributes: HashMap::new(),
            data: None,
        };

        assert_eq!(ce.id, "CE_ID");
    }
}
