/// CAPL Token definitions for MVP analyzer
/// Covers C-like syntax + CAPL-specific keywords and constructs

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // ========== Keywords ==========
    // Control flow
    If,
    Else,
    Switch,
    Case,
    Default,
    For,
    While,
    Do,
    Break,
    Continue,
    Return,

    // CAPL event keywords
    On,
    Start,
    PreStart,
    PreStop,
    StopMeasurement,
    Key,
    EnvVar,
    SysVar,
    SysVarChange,
    SysVarUpdate,
    Signal,
    SignalUpdate,
    Pdu,

    // ========== Data Types ==========
    // Integer types
    Byte,
    Word,
    Dword,
    Int,
    Long,
    Int64,
    Qword,
    
    // Floating point
    Float,
    Double,
    
    // Character
    Char,
    
    // Structural
    Struct,
    Enum,
    
    // CAPL-specific types
    Timer,
    MsTimer,
    Message,
    
    // Class types (File, TcpSocket, etc.)
    File,
    TcpSocket,
    UdpSocket,
    
    // Pointer-related keywords
    DbMsg,
    SignalPtr,
    SysvarPtr,

    // Type qualifiers
    Const,
    
    // ========== Identifiers & Literals ==========
    Identifier(String),
    
    // Number literals
    IntLiteral(i64),
    FloatLiteral(f64),
    HexLiteral(String),    // 0x123, 0xABCD
    
    // Character and String
    CharLiteral(char),
    StringLiteral(String),

    // ========== Operators ==========
    // Arithmetic
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    
    // Increment/Decrement
    PlusPlus,       // ++
    MinusMinus,     // --
    
    // Comparison
    Equal,          // ==
    NotEqual,       // !=
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=
    
    // Logical
    LogicalAnd,     // &&
    LogicalOr,      // ||
    LogicalNot,     // !
    
    // Bitwise
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LeftShift,      // <<
    RightShift,     // >>
    
    // Assignment
    Assign,         // =
    PlusAssign,     // +=
    MinusAssign,    // -=
    StarAssign,     // *=
    SlashAssign,    // /=
    PercentAssign,  // %=
    AndAssign,      // &=
    OrAssign,       // |=
    XorAssign,      // ^=
    LeftShiftAssign,  // <<=
    RightShiftAssign, // >>=
    
    // Ternary
    Question,       // ?
    Colon,          // :
    
    // Member access
    Dot,            // .
    Arrow,          // ->
    
    // ========== Delimiters ==========
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    
    Semicolon,      // ;
    Comma,          // ,
    
    // ========== Special ==========
    Eof,            // End of file
    Unknown(char),  // Unknown character
}

impl Token {
    /// Check if token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            Token::If | Token::Else | Token::Switch | Token::Case | Token::Default
            | Token::For | Token::While | Token::Do | Token::Break | Token::Continue
            | Token::Return | Token::On | Token::Start | Token::PreStart | Token::PreStop
            | Token::StopMeasurement | Token::Key | Token::EnvVar | Token::SysVar
            | Token::SysVarChange | Token::SysVarUpdate | Token::Signal | Token::SignalUpdate
            | Token::Pdu | Token::Struct | Token::Enum | Token::Const
        )
    }

    /// Check if token is a type keyword
    pub fn is_type(&self) -> bool {
        matches!(
            self,
            Token::Byte | Token::Word | Token::Dword | Token::Int | Token::Long
            | Token::Int64 | Token::Qword | Token::Float | Token::Double | Token::Char
            | Token::Struct | Token::Enum | Token::Timer | Token::MsTimer | Token::Message
            | Token::File | Token::TcpSocket | Token::UdpSocket
            | Token::DbMsg | Token::SignalPtr | Token::SysvarPtr
        )
    }

    /// Check if token is an assignment operator
    pub fn is_assignment(&self) -> bool {
        matches!(
            self,
            Token::Assign | Token::PlusAssign | Token::MinusAssign | Token::StarAssign
            | Token::SlashAssign | Token::PercentAssign | Token::AndAssign | Token::OrAssign
            | Token::XorAssign | Token::LeftShiftAssign | Token::RightShiftAssign
        )
    }

    /// Get keyword from string
    pub fn from_keyword(keyword: &str) -> Option<Token> {
        match keyword {
            // Control flow
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "switch" => Some(Token::Switch),
            "case" => Some(Token::Case),
            "default" => Some(Token::Default),
            "for" => Some(Token::For),
            "while" => Some(Token::While),
            "do" => Some(Token::Do),
            "break" => Some(Token::Break),
            "continue" => Some(Token::Continue),
            "return" => Some(Token::Return),

            // CAPL events
            "on" => Some(Token::On),
            "start" => Some(Token::Start),
            "preStart" => Some(Token::PreStart),
            "preStop" => Some(Token::PreStop),
            "stopMeasurement" => Some(Token::StopMeasurement),
            "key" => Some(Token::Key),
            "envVar" => Some(Token::EnvVar),
            "sysVar" => Some(Token::SysVar),
            "sysVar_change" => Some(Token::SysVarChange),
            "sysVar_update" => Some(Token::SysVarUpdate),
            "signal" => Some(Token::Signal),
            "signal_update" => Some(Token::SignalUpdate),
            "PDU" => Some(Token::Pdu),

            // Data types
            "byte" => Some(Token::Byte),
            "word" => Some(Token::Word),
            "dword" => Some(Token::Dword),
            "int" => Some(Token::Int),
            "long" => Some(Token::Long),
            "int64" => Some(Token::Int64),
            "qword" => Some(Token::Qword),
            "float" => Some(Token::Float),
            "double" => Some(Token::Double),
            "char" => Some(Token::Char),
            "struct" => Some(Token::Struct),
            "enum" => Some(Token::Enum),
            "timer" => Some(Token::Timer),
            "msTimer" => Some(Token::MsTimer),
            "message" => Some(Token::Message),
            
            // Class types
            "File" => Some(Token::File),
            "TcpSocket" => Some(Token::TcpSocket),
            "UdpSocket" => Some(Token::UdpSocket),
            
            // Pointer types
            "dbMsg" => Some(Token::DbMsg),
            "Signal" => Some(Token::SignalPtr),
            "sysvar" => Some(Token::SysvarPtr),

            // Qualifiers
            "const" => Some(Token::Const),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_recognition() {
        assert_eq!(Token::from_keyword("if"), Some(Token::If));
        assert_eq!(Token::from_keyword("on"), Some(Token::On));
        assert_eq!(Token::from_keyword("timer"), Some(Token::Timer));
        assert_eq!(Token::from_keyword("not_a_keyword"), None);
    }

    #[test]
    fn test_is_type() {
        assert!(Token::Int.is_type());
        assert!(Token::Message.is_type());
        assert!(!Token::If.is_type());
    }

    #[test]
    fn test_is_assignment() {
        assert!(Token::Assign.is_assignment());
        assert!(Token::PlusAssign.is_assignment());
        assert!(!Token::Plus.is_assignment());
    }
}
