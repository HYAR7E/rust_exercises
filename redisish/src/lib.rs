#[derive(Debug, PartialEq)]
pub enum Protocol<'a> {
    PUBLISH { message: &'a str },
    RETRIEVE,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnknownProtocol,
    MissingNewline,
    DataAfterNewline,
    ProtocolRequiresSpace,
    UnexpectedPayload,
    MissingPayload,
}

pub fn parse(input: &str) -> Result<Protocol, ParseError> {
    match input.split_once('\n') {
        Some(("RETRIEVE", "")) => Ok(Protocol::RETRIEVE),
        Some((command, "")) => match command.split_once(' ') {
            Some(("PUBLISH", message)) => Ok(Protocol::PUBLISH { message }),
            Some(("RETRIEVE", _)) => Err(ParseError::UnexpectedPayload),
            Some(_) => Err(ParseError::UnknownProtocol),
            None => Err(ParseError::MissingPayload),
        },
        Some((_, _)) => Err(ParseError::DataAfterNewline),
        None => Err(ParseError::MissingNewline),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_message() {
        assert_eq!(
            parse("PUBLISH Hello world!\n"),
            Ok(Protocol::PUBLISH {
                message: "Hello world!",
            })
        );
    }

    #[test]
    fn test_publish_empty_message() {
        assert_eq!(parse("PUBLISH \n"), Ok(Protocol::PUBLISH { message: "" }));
    }

    #[test]
    fn test_retrieve() {
        assert_eq!(parse("RETRIEVE\n"), Ok(Protocol::RETRIEVE));
    }

    #[test]
    fn fail_retrieve_unexpected_payload() {
        assert_eq!(
            parse("RETRIEVE not required\n"),
            Err(ParseError::UnexpectedPayload)
        );
    }

    #[test]
    fn fail_with_no_newline() {
        assert_eq!(parse("RETRIEVE"), Err(ParseError::MissingNewline));
    }

    #[test]
    fn fail_with_data_after_newline() {
        assert_eq!(parse("PUBLISH \n123"), Err(ParseError::DataAfterNewline));
    }

    #[test]
    fn fail_unknown_protocol() {
        assert_eq!(parse("SEND hola\n"), Err(ParseError::UnknownProtocol));
    }
}
