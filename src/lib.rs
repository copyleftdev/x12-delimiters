pub mod errors;

use errors::DelimiterError;

const DEFAULT_SEGMENT_TERMINATOR: u8 = b'~';
const DEFAULT_ELEMENT_SEPARATOR: u8 = b'*';
const DEFAULT_SUB_ELEMENT_SEPARATOR: u8 = b':';

const ISA_MIN_LENGTH: usize = 106;
const ISA_ELEMENT_SEPARATOR_INDEX: usize = 3;
const ISA_SUB_ELEMENT_SEPARATOR_INDEX: usize = 104;
const ISA_SEGMENT_TERMINATOR_INDEX: usize = 105;

/// Represents the three delimiter types used in X12 EDI transactions.
///
/// X12 delimiters control how segments, elements, and sub-elements are separated in the EDI data.
/// The standard default delimiters are:
/// - Segment terminator: `~`
/// - Element separator: `*`
/// - Sub-element separator: `:`
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Delimiters {
    segment_terminator: u8,
    element_separator: u8,
    sub_element_separator: u8,
}

impl Delimiters {
    /// Creates a new Delimiters instance with the specified values.
    ///
    /// # Arguments
    /// * `segment_terminator` - Character used to terminate segments
    /// * `element_separator` - Character used to separate elements
    /// * `sub_element_separator` - Character used to separate sub-elements
    pub fn new(segment_terminator: u8, element_separator: u8, sub_element_separator: u8) -> Self {
        Delimiters {
            segment_terminator,
            element_separator,
            sub_element_separator,
        }
    }

    /// Extracts delimiters from an ISA segment.
    ///
    /// The ISA segment is the first segment in an X12 file and contains the delimiter information.
    /// - Element separator is at position 3
    /// - Sub-element separator is at position 104
    /// - Segment terminator is at position 105
    ///
    /// # Arguments
    /// * `isa_segment` - Byte slice containing the ISA segment
    ///
    /// # Returns
    /// * `Result<Delimiters, DelimiterError>` - Delimiters on success, error on failure
    ///
    /// # Errors
    /// Returns `DelimiterError::InvalidIsaLength` if the ISA segment is too short
    pub fn from_isa(isa_segment: &[u8]) -> Result<Self, DelimiterError> {
        if isa_segment.len() < ISA_MIN_LENGTH {
            return Err(DelimiterError::InvalidIsaLength);
        }

        let element_separator = isa_segment[ISA_ELEMENT_SEPARATOR_INDEX];
        let sub_element_separator = isa_segment[ISA_SUB_ELEMENT_SEPARATOR_INDEX];
        let segment_terminator = isa_segment[ISA_SEGMENT_TERMINATOR_INDEX];

        Ok(Delimiters {
            element_separator,
            sub_element_separator,
            segment_terminator,
        })
    }

    /// Returns the segment terminator character.
    pub fn segment_terminator(&self) -> u8 {
        self.segment_terminator
    }

    /// Returns the element separator character.
    pub fn element_separator(&self) -> u8 {
        self.element_separator
    }

    /// Returns the sub-element separator character.
    pub fn sub_element_separator(&self) -> u8 {
        self.sub_element_separator
    }

    /// Validates that all three delimiters are distinct.
    ///
    /// In X12 EDI, all delimiters must be different characters to avoid ambiguity.
    ///
    /// # Returns
    /// * `bool` - True if all delimiters are unique, false otherwise
    pub fn are_valid(&self) -> bool {
        self.segment_terminator != self.element_separator &&
        self.segment_terminator != self.sub_element_separator &&
        self.element_separator != self.sub_element_separator
    }
}

impl Default for Delimiters {
    /// Creates a Delimiters instance with the standard default values.
    fn default() -> Self {
        Delimiters {
            segment_terminator: DEFAULT_SEGMENT_TERMINATOR,
            element_separator: DEFAULT_ELEMENT_SEPARATOR,
            sub_element_separator: DEFAULT_SUB_ELEMENT_SEPARATOR,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_ISA_SEGMENT_STANDARD: &[u8] = b"ISA*00*          *00*          *ZZ*SENDERID       *ZZ*RECEIVERID     *250403*0856*U*00501*000000001*0*P*:~";
    const SAMPLE_ISA_SEGMENT_ALT: &[u8] = b"ISA^00^          ^00^          ^ZZ^SENDERID       ^ZZ^RECEIVERID     ^250403^0856^U^00401^000000002^1^T^>}";
    const TOO_SHORT_ISA: &[u8] = b"ISA*00*";

    #[test]
    fn test_default_delimiters() {
        let delimiters = Delimiters::default();
        assert_eq!(delimiters.segment_terminator(), b'~');
        assert_eq!(delimiters.element_separator(), b'*');
        assert_eq!(delimiters.sub_element_separator(), b':');
    }

    #[test]
    fn test_new_delimiters() {
        let delimiters = Delimiters::new(b'!', b'@', b'#');
        assert_eq!(delimiters.segment_terminator(), b'!');
        assert_eq!(delimiters.element_separator(), b'@');
        assert_eq!(delimiters.sub_element_separator(), b'#');
    }

    #[test]
    fn test_from_isa_standard() {
        let result = Delimiters::from_isa(SAMPLE_ISA_SEGMENT_STANDARD);
        assert!(result.is_ok());
        let delimiters = result.unwrap();
        assert_eq!(delimiters.segment_terminator(), b'~');
        assert_eq!(delimiters.element_separator(), b'*');
        assert_eq!(delimiters.sub_element_separator(), b':');
    }

    #[test]
    fn test_from_isa_alternative() {
        let result = Delimiters::from_isa(SAMPLE_ISA_SEGMENT_ALT);
        assert!(result.is_ok());
        let delimiters = result.unwrap();
        assert_eq!(delimiters.segment_terminator(), b'}');
        assert_eq!(delimiters.element_separator(), b'^');
        assert_eq!(delimiters.sub_element_separator(), b'>');
    }

    #[test]
    fn test_from_isa_too_short() {
        let result = Delimiters::from_isa(TOO_SHORT_ISA);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), DelimiterError::InvalidIsaLength);
    }

    #[test]
    fn test_from_isa_exact_length() {
        let exact_len_isa = SAMPLE_ISA_SEGMENT_STANDARD[..ISA_MIN_LENGTH].to_vec();
        assert_eq!(exact_len_isa.len(), ISA_MIN_LENGTH);
        
        let result = Delimiters::from_isa(&exact_len_isa);
        assert!(result.is_ok());
        let delimiters = result.unwrap();
        assert_eq!(delimiters.segment_terminator(), b'~');
        assert_eq!(delimiters.element_separator(), b'*');
        assert_eq!(delimiters.sub_element_separator(), b':');
    }

    #[test]
    fn test_getters() {
        let delimiters = Delimiters::new(b'A', b'B', b'C');
        assert_eq!(delimiters.segment_terminator(), b'A');
        assert_eq!(delimiters.element_separator(), b'B');
        assert_eq!(delimiters.sub_element_separator(), b'C');
    }

    #[test]
    fn test_are_valid() {
        let valid_delimiters = Delimiters::new(b'~', b'*', b':');
        assert!(valid_delimiters.are_valid());
        
        let invalid_delimiters1 = Delimiters::new(b'*', b'*', b':'); 
        assert!(!invalid_delimiters1.are_valid());
        
        let invalid_delimiters2 = Delimiters::new(b'~', b'*', b'*');
        assert!(!invalid_delimiters2.are_valid());
        
        let invalid_delimiters3 = Delimiters::new(b'~', b'~', b':');
        assert!(!invalid_delimiters3.are_valid());
    }

    use proptest::prelude::*;

    fn valid_delimiter() -> impl Strategy<Value = u8> {
        (33..=126u8).prop_filter("Avoiding whitespace", |&c| c != b' ' && c != b'\t' && c != b'\n' && c != b'\r')
    }

    fn distinct_delimiters() -> impl Strategy<Value = (u8, u8, u8)> {
        (valid_delimiter(), valid_delimiter(), valid_delimiter())
            .prop_filter("Delimiters must be distinct", |(a, b, c)| a != b && b != c && a != c)
    }

    fn isa_segment_with_delimiters() -> impl Strategy<Value = (Vec<u8>, u8, u8, u8)> {
        distinct_delimiters().prop_flat_map(|(elem_sep, sub_elem_sep, seg_term)| {
            let mut isa = Vec::with_capacity(ISA_MIN_LENGTH);
            isa.extend_from_slice(b"ISA");
            isa.push(elem_sep);
            
            for i in 4..ISA_SUB_ELEMENT_SEPARATOR_INDEX {
                if i % 2 == 0 {
                    isa.push(elem_sep);
                } else {
                    isa.push(b'X');
                }
            }
            
            while isa.len() < ISA_SUB_ELEMENT_SEPARATOR_INDEX {
                isa.push(b'X');
            }
            
            isa.push(sub_elem_sep);
            isa.push(seg_term);
            
            Just((isa, elem_sep, sub_elem_sep, seg_term))
        })
    }

    fn isa_segment_extended() -> impl Strategy<Value = (Vec<u8>, u8, u8, u8)> {
        isa_segment_with_delimiters().prop_flat_map(|(isa, elem_sep, sub_elem_sep, seg_term)| {
            (0..=10).prop_map(move |n| {
                let mut extended_isa = isa.clone();
                for _ in 0..n {
                    extended_isa.push(b'X');
                }
                (extended_isa, elem_sep, sub_elem_sep, seg_term)
            })
        })
    }

    fn invalid_length_isa() -> impl Strategy<Value = Vec<u8>> {
        (1..ISA_MIN_LENGTH).prop_map(|len| {
            let mut isa = Vec::with_capacity(len);
            isa.extend_from_slice(b"ISA*"); 
            while isa.len() < len {
                isa.push(b'X');
            }
            isa
        })
    }

    proptest! {
        #[test]
        fn prop_from_isa_extracts_correct_delimiters(
            (isa, elem_sep, sub_elem_sep, seg_term) in isa_segment_with_delimiters()
        ) {
            let result = Delimiters::from_isa(&isa);
            prop_assert!(result.is_ok(), "from_isa should succeed on valid ISA segment");
            
            let delimiters = result.unwrap();
            prop_assert_eq!(delimiters.element_separator(), elem_sep);
            prop_assert_eq!(delimiters.sub_element_separator(), sub_elem_sep);
            prop_assert_eq!(delimiters.segment_terminator(), seg_term);
        }

        #[test]
        fn prop_from_isa_works_with_extended_segments(
            (isa, elem_sep, sub_elem_sep, seg_term) in isa_segment_extended()
        ) {
            let result = Delimiters::from_isa(&isa);
            prop_assert!(result.is_ok(), "from_isa should succeed on extended ISA segment");
            
            let delimiters = result.unwrap();
            prop_assert_eq!(delimiters.element_separator(), elem_sep);
            prop_assert_eq!(delimiters.sub_element_separator(), sub_elem_sep);
            prop_assert_eq!(delimiters.segment_terminator(), seg_term);
        }

        #[test]
        fn prop_new_delimiters_preserves_values(
            (seg_term, elem_sep, sub_elem_sep) in distinct_delimiters()
        ) {
            let delimiters = Delimiters::new(seg_term, elem_sep, sub_elem_sep);
            prop_assert_eq!(delimiters.segment_terminator(), seg_term);
            prop_assert_eq!(delimiters.element_separator(), elem_sep);
            prop_assert_eq!(delimiters.sub_element_separator(), sub_elem_sep);
        }

        #[test]
        fn prop_delimiter_roundtrip(
            (seg_term, elem_sep, sub_elem_sep) in distinct_delimiters()
        ) {
            let delimiters1 = Delimiters::new(seg_term, elem_sep, sub_elem_sep);
            
            let delimiters2 = Delimiters::new(
                delimiters1.segment_terminator(),
                delimiters1.element_separator(),
                delimiters1.sub_element_separator()
            );
            
            prop_assert_eq!(delimiters1, delimiters2);
        }

        #[test]
        fn prop_delimiter_equality(
            (s1, e1, se1) in distinct_delimiters(), 
            (s2, e2, se2) in distinct_delimiters()
        ) {
            let d1 = Delimiters::new(s1, e1, se1);
            let d2 = Delimiters::new(s1, e1, se1);
            let d3 = Delimiters::new(s2, e2, se2);

            prop_assert_eq!(d1, d2);
            
            if s1 != s2 || e1 != e2 || se1 != se2 {
                prop_assert_ne!(d1, d3);
            }
        }

        #[test]
        fn prop_invalid_length_isa_returns_error(
            isa in invalid_length_isa()
        ) {
            let result = Delimiters::from_isa(&isa);
            prop_assert!(result.is_err());
            prop_assert_eq!(result.err().unwrap(), DelimiterError::InvalidIsaLength);
        }

        #[test]
        fn prop_valid_delimiters_check(
            (seg_term, elem_sep, sub_elem_sep) in distinct_delimiters()
        ) {
            let valid = Delimiters::new(seg_term, elem_sep, sub_elem_sep);
            prop_assert!(valid.are_valid());
            
            let invalid1 = Delimiters::new(seg_term, seg_term, sub_elem_sep);
            prop_assert!(!invalid1.are_valid());
            
            let invalid2 = Delimiters::new(seg_term, elem_sep, seg_term);
            prop_assert!(!invalid2.are_valid());
            
            let invalid3 = Delimiters::new(seg_term, elem_sep, elem_sep);
            prop_assert!(!invalid3.are_valid());
        }
    }
}