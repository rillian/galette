
pub struct Error {
    pub code: ErrorCode,
    pub line: i32,
}

pub enum ErrorCode {
    Code(i32),
}

const ERROR_CODES: [&str; 49] = [
    "error in source file found",
    "Line  1: type of GAL expected",
    "unexpected end of file",
    "pinname expected after '/'",
    "max. length of pinname is 8 characters",
    "illegal character in pin declaration",
    "illegal VCC/GND assignment",
    "pin declaration: expected VCC at VCC pin",
    "pin declaration: expected GND at GND pin",
    "pinname defined twice",
    "illegal use of '/'",
    "unknown pinname",
    "NC (Not Connected) is not allowed in logic equations",
    "unknown suffix found",
    "'=' expected",
    "this pin can't be used as output",
    "same pin is defined multible as output",
    "before using .E, the output must be defined",
    "GAL22V10: AR and SP is not allowed as pinname",
    ".E, .CLK, .ARST and .APRST is not allowed to be negated",
    "mode 2: pins 12, 19 can't be used as input",
    "mode 2: pins 15, 22 can't be used as input",
    "tristate control is defined twice",
    "GAL16V8/20V8: tri. control for reg. output is not allowed",
    "tristate control without previous '.T'",
    "use GND, VCC instead of /VCC, /GND",
    "pin not allowed in equations",
    "mode 3: pins 1,13 are reserved for 'Clock' and '/OE'",
    "use of VCC and GND is not allowed in equations",
    "only one product term allowed (no OR)",
    "too many product terms",
    "use of AR and SP is not allowed in equations",
    "negation of AR and SP is not allowed",
    "no equations found",
    ".CLK is not allowed when this type of GAL is used",
    ".ARST is not allowed when this type of GAL is used",
    ".APRST is not allowed when this type of GAL is used",
    "GAL20RA10: pin 1 can't be used in equations",
    "GAL20RA10: pin 13 can't be used in equations",
    "AR, SP: no suffix allowed",
    "AR or SP is defined twice",
    "missing clock definition (.CLK) of registered output",
    "before using .CLK, the output must be defined",
    "before using .ARST, the output must be defined",
    "before using .APRST the output must be defined",
    "several .CLK definitions for the same output found",
    "several .ARST definitions for the same output found",
    "several .APRST definitions for the same output found",
    "use of .CLK, .ARST, .APRST only allowed for registered outputs"
];

fn error_string(err_code: ErrorCode) -> &'static str {
    match err_code {
        ErrorCode::Code(i) => ERROR_CODES[i as usize],
    }
}

pub fn print_error(err: Error) {
    println!("Error in line {}: {}", err.line, error_string(err.code));
}