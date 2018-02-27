# Unicode

| name                     | from |     to |
|--------------------------|-----:|-------:|
| Unicode Code Point       |    0 | 10FFFF |
| ASCII                    |    0 |     7F |
| Unicode Scalar Value (1) |    0 |   D7FF |
| Surrogate Code Point     | D800 |   DFFF |
| Unicode Scalar Value (2) | E000 | 10FFFF |
| Byte Order Mark (BOM)    | FEFF |        |


http://www.unicode.org/glossary/


## Code Point Type
Any of the 7 fundamental classes of code points in the standard: 
- Graphic
- Format
- Control
- Private-Use
- Surrogate
- Noncharacter
- Reserved

## Unicode Code Point
1. Any value in the Unicode codespace, from 0 to U+10FFFF; not all code points are assigned to encoded characters.
2. A value, or position, for a character, in any coded character set.

## Unicode Scalar Value
Any Unicode code point except high-surrogate and low-surrogate code points. It is in the range: 0 to U+D7FF and U+E000 to 10FFFF inclusive.

## Surrogate Code Point
A Unicode code point in the range U+D800 - U+DFFF. Reserved for use by UTF-16, where a surrogate pair "stands in" for a supplementary code point.

## Surrogate Pair
A representation for a single abstract character - a sequence of two 16-bit code units, where the first value of the pair is a high-surrogate code unit, and the second is a low-surrogate code unit.




## ASCII
The set of 128 Unicode characters from U+0000 to U+007F, including control codes as well as graphic characters. ASCII completely maps to UTF-8 Unicode encoding; this means that any ASCII encoded text is valid UTF-8.

## Byte Order Mark (BOM)
The Unicode character U+FEFF when used to indicate the byte order of a text. 

## Canonical
1. Conforming to the general rules for encoding—that is, not compressed, compacted, or in any other form specified by a higher protocol.
2. Characteristic of a normative mapping and form of equivalence specified in Chapter 3, Conformance.

## Canonical Composition
A step in the algorithm for Unicode Normalization Forms, during which decomposed sequences are replaced by primary composites, where possible. (See definition D115 in Section 3.11, Normalization Forms.)

## Canonical Decomposable Character
A character that is not identical to its canonical decomposition. (See definition D69 in Section 3.7. Decomposition.)

## Canonical Decomposition
Mapping to an inherently equivalent sequence—for example, mapping ä to a +  combining umlaut. (For a full, formal definition, see definition D68 in Section 3.7. Decomposition.)

## Canonical Equivalent
Two character sequences are said to be canonical equivalents if their full canonical decompositions are identical. (See definition D70 in Section 3.7. Decomposition.)

## Character
1. The smallest component of written language that has semantic value; refers to the abstract meaning and/or shape, rather than a specific shape, though in code tables some form of visual representation is essential for the reader’s understanding.
2. Synonym for abstract character.
3. The basic unit of encoding for the Unicode character encoding.
4. The English name for the ideographic written elements of Chinese origin.

## Glyph
1. An abstract form that represents one or more glyph images.
2. A synonym for glyph image. In displaying Unicode character data, one or more glyphs may be selected to depict a particular character. These glyphs are selected by a rendering engine during composition and layout processing.






## 3.4 Characters and Encoding

### Abstract character
a unit of information used for the organization, control, or representation of textual data.
- When representing data, the nature of that data is generally symbolic as
opposed to some other kind of data (for example, aural or visual). Examples of
such symbolic data include letters, ideographs, digits, punctuation, technical
symbols, and dingbats.
• An abstract character has no concrete form and should not be confused with a
glyph.
• An abstract character does not necessarily correspond to what a user thinks of
as a “character” and should not be confused with a grapheme.
• The abstract characters encoded by the Unicode Standard are known as Unicode
abstract characters.
• Abstract characters not directly encoded by the Unicode Standard can often be
represented by the use of combining character sequences.
D8 Abstract character sequence: An ordered sequence of one or more abstract characters.
D9 Unicode codespace: A range of integers from 0 to 10FFFF16.
• This particular range is defined for the codespace in the Unicode Standard.
Other character encoding standards may use other codespaces.
D10 Code point: Any value in the Unicode codespace.
• A code point is also known as a code position.
• See D77 for the definition of code unit.
D10a Code point type: Any of the seven fundamental classes of code points in the standard:
Graphic, Format, Control, Private-Use, Surrogate, Noncharacter, Reserved.
• See Table 2-3 for a summary of the meaning and use of each class.
• For Noncharacter, see also D14 Noncharacter.
• For Reserved, see also D15 Reserved code point.
• For Private-Use, see also D49 Private-use code point.
• For Surrogate, see also D71 High-surrogate code point and D73 Low-surrogate
code point.
D10b Block: A named range of code points used to organize the allocation of characters.
• The exact list of blocks defined for each version of the Unicode Standard is
specified by the data file Blocks.txt in the Unicode Character Database.
Conformance 91 3.4 Characters and Encoding
• The range for each defined block is specified by Field 0 in Blocks.txt; for example,
“0000..007F”.
• The ranges for blocks are non-overlapping. In other words, no code point can
be contained in the range for one block and also in the range for a second distinct
block.
• The range for each block is defined as a contiguous sequence. In other words, a
block cannot consist of two (or more) discontiguous sequences of code points.
• Each range for a defined block starts with a value for which code point MOD 16
= 0 and terminates with a larger value for which code point MOD 16 = 15. This
specification results in block ranges which always include full code point columns
for code chart display. A block never starts or terminates in mid-column.
• All assigned characters are contained within ranges for defined blocks.
• Blocks may contain reserved code points, but no block contains only reserved
code points. The majority of reserved code points are outside the ranges of
defined blocks.
• A few designated code points are not contained within the ranges for defined
blocks. This applies to the noncharacter code points at the last two code points
of supplementary planes 1 through 14.
• The name for each defined block is specified by Field 1 in Blocks.txt; for example,
“Basic Latin”.
• The names for defined blocks constitute a unique namespace.
• The uniqueness rule for the block namespace is LM3, as defined in Unicode
Standard Annex #44, “Unicode Character Database.” In other words, casing,
white space, hyphens, and underscores are ignored when matching strings for
block names. The string “BASIC LATIN” or “Basic_Latin” would be considered
as matching the name for the block named “Basic Latin”.
• There is also a normative Block property. See Table 3-2. The Block property is a
catalog property whose value is a string that identifies a block.
• Property value aliases for the Block property are defined in PropertyValueAliases.txt
in the Unicode Character Database. The long alias defined for the
Block property is always a loose match for the name of the block defined in
Blocks.txt. Additional short aliases and other aliases are provided for convenience
of use in regular expression syntax.
• The default value for the Block property is “No_Block”. This default applies to
any code point which is not contained in the range of a defined block.
For a general discussion of blocks and their relation to allocation in the Unicode Standard,
see “Allocation Areas and Blocks” in Section 2.8, Unicode Allocation. For a general discus-
Conformance 92 3.4 Characters and Encoding
sion of the use of blocks in the presentation of the Unicode code charts, see Chapter 24,
About the Code Charts.
D11 Encoded character: An association (or mapping) between an abstract character and a
code point.
• An encoded character is also referred to as a coded character.
• While an encoded character is formally defined in terms of the mapping
between an abstract character and a code point, informally it can be thought of
as an abstract character taken together with its assigned code point.
• Occasionally, for compatibility with other standards, a single abstract character
may correspond to more than one code point—for example, “Å” corresponds
both to U+00C5 Å latin capital letter a with ring above and to U+212B
Å angstrom sign.
• A single abstract character may also be represented by a sequence of code
points—for example, latin capital letter g with acute may be represented by the
sequence <U+0047 latin capital letter g, U+0301 combining acute
accent>, rather than being mapped to a single code point.
D12 Coded character sequence: An ordered sequence of one or more code points.
• A coded character sequence is also known as a coded character representation.
• Normally a coded character sequence consists of a sequence of encoded characters,
but it may also include noncharacters or reserved code points.
• Internally, a process may choose to make use of noncharacter code points in its
coded character sequences. However, such noncharacter code points may not
be interpreted as abstract characters (see conformance clause C2). Their
removal by a conformant process constitutes modification of interpretation of
the coded character sequence (see conformance clause C7).
• Reserved code points are included in coded character sequences, so that the
conformance requirements regarding interpretation and modification are
properly defined when a Unicode-conformant implementation encounters
coded character sequences produced under a future version of the standard.
Unless specified otherwise for clarity, in the text of the Unicode Standard the term character
alone designates an encoded character. Similarly, the term character sequence alone designates
a coded character sequence.
D13 Deprecated character: A coded character whose use is strongly discouraged.
• Deprecated characters are retained in the standard indefinitely, but should not
be used. They are retained in the standard so that previously conforming data
stay conformant in future versions of the standard.
• Deprecated characters typically consist of characters with significant architectural
problems, or ones which cause implementation problems. Some examples
Conformance 93 3.4 Characters and Encoding
of characters deprecated on these grounds include tag characters (see
Section 23.9, Tag Characters) and the alternate format characters (see
Section 23.3, Deprecated Format Characters).
• Deprecated characters are explicitly indicated in the Unicode code charts. They
are also given an explicit property value of Deprecated=True in the Unicode
Character Database.
• Deprecated characters should not be confused with obsolete characters, which
are historical. Obsolete characters do not occur in modern text, but they are
not deprecated; their use is not discouraged.
D14 Noncharacter: A code point that is permanently reserved for internal use. Noncharacters
consist of the values U+nFFFE and U+nFFFF (where n is from 0 to 1016) and
the values U+FDD0..U+FDEF.
• For more information, see Section 23.7, Noncharacters.
• These code points are permanently reserved as noncharacters.
D15 Reserved code point: Any code point of the Unicode Standard that is reserved for
future assignment. Also known as an unassigned code point.
• Surrogate code points and noncharacters are considered assigned code points,
but not assigned characters.
• For a summary classification of reserved and other types of code points, see
Table 2-3.
In general, a conforming process may indicate the presence of a code point whose use has
not been designated (for example, by showing a missing glyph in rendering or by signaling
an appropriate error in a streaming protocol), even though it is forbidden by the standard
from interpreting that code point as an abstract character.
D16 Higher-level protocol: Any agreement on the interpretation of Unicode characters
that extends beyond the scope of this standard.
• Such an agreement need not be formally announced in data; it may be implicit
in the context.
• The specification of some Unicode algorithms may limit the scope of what a
conformant higher-level protocol may do.
D17 Unicode algorithm: The logical description of a process used to achieve a specified
result involving Unicode characters.
• This definition, as used in the Unicode Standard and other publications of the
Unicode Consortium, is intentionally broad so as to allow precise logical
description of required results, without constraining implementations to follow
the precise steps of that logical description.
Conformance 94 3.4 Characters and Encoding
D18 Named Unicode algorithm: A Unicode algorithm that is specified in the Unicode
Standard or in other standards published by the Unicode Consortium and that is
given an explicit name for ease of reference.
• Named Unicode algorithms are cited in titlecase in the Unicode Standard.
Table 3-1 lists the named Unicode algorithms and indicates the locations of their specifications.
Details regarding conformance to these algorithms and any restrictions they place on
the scope of allowable tailoring by higher-level protocols can be found in the specifications.
In some cases, a named Unicode algorithm is provided for information only. When externally
referenced, a named Unicode algorithm may be prefixed with the qualifier “Unicode”
to make the connection of the algorithm to the Unicode Standard and other Unicode specifications
clear. Thus, for example, the Bidirectional Algorithm is generally referred to by its
full name, “Unicode Bidirectional Algorithm.” As much as is practical, the titles of Unicode
Standard Annexes which define Unicode algorithms consist of the name of the Unicode
algorithm they specify. In a few cases, named Unicode algorithms are also widely known by
their acronyms, and those acronyms are also listed in 