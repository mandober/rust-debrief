# Crate core

List of all items in `core` crate
Version 1.28.0-nightly (c2d46037f 2018-05-24)

<!-- TOC -->

- [Structs](#structs)
- [Enums](#enums)
- [Unions](#unions)
- [Traits](#traits)
- [Macros](#macros)
- [Functions](#functions)
- [Typedefs](#typedefs)
- [Constants](#constants)

<!-- /TOC -->


## Structs
  alloc::AllocErr
  alloc::CannotReallocInPlace
  alloc::Excess
  alloc::Layout
  alloc::LayoutErr
  any::TypeId
  - arch
    arch::aarch64::float32x2_t
    arch::aarch64::float32x4_t
    arch::aarch64::float64x1_t
    arch::aarch64::float64x2_t
    arch::aarch64::int16x4_t
    arch::aarch64::int16x8_t
    arch::aarch64::int32x2_t
    arch::aarch64::int32x4_t
    arch::aarch64::int64x1_t
    arch::aarch64::int64x2_t
    arch::aarch64::int8x16_t
    arch::aarch64::int8x8_t
    arch::aarch64::poly16x4_t
    arch::aarch64::poly16x8_t
    arch::aarch64::poly8x16_t
    arch::aarch64::poly8x8_t
    arch::aarch64::uint16x4_t
    arch::aarch64::uint16x8_t
    arch::aarch64::uint32x2_t
    arch::aarch64::uint32x4_t
    arch::aarch64::uint64x1_t
    arch::aarch64::uint64x2_t
    arch::aarch64::uint8x16_t
    arch::aarch64::uint8x8_t
    arch::arm::float32x2_t
    arch::arm::float32x4_t
    arch::arm::int16x4_t
    arch::arm::int16x8_t
    arch::arm::int32x2_t
    arch::arm::int32x4_t
    arch::arm::int64x1_t
    arch::arm::int64x2_t
    arch::arm::int8x16_t
    arch::arm::int8x8_t
    arch::arm::poly16x4_t
    arch::arm::poly16x8_t
    arch::arm::poly8x16_t
    arch::arm::poly8x8_t
    arch::arm::uint16x4_t
    arch::arm::uint16x8_t
    arch::arm::uint32x2_t
    arch::arm::uint32x4_t
    arch::arm::uint64x1_t
    arch::arm::uint64x2_t
    arch::arm::uint8x16_t
    arch::arm::uint8x8_t
    arch::x86::CpuidResult
    arch::x86::__m128
    arch::x86::__m128d
    arch::x86::__m128i
    arch::x86::__m256
    arch::x86::__m256d
    arch::x86::__m256i
    arch::x86::__m64
    arch::x86_64::CpuidResult
    arch::x86_64::__m128
    arch::x86_64::__m128d
    arch::x86_64::__m128i
    arch::x86_64::__m256
    arch::x86_64::__m256d
    arch::x86_64::__m256i
    arch::x86_64::__m64
  array::TryFromSliceError
  ascii::EscapeDefault
  cell::BorrowError
  cell::BorrowMutError
  cell::Cell
  cell::Ref
  cell::RefCell
  cell::RefMut
  cell::UnsafeCell
  char::CharTryFromError
  char::DecodeUtf16
  char::DecodeUtf16Error
  char::DecodeUtf8
  char::EscapeDebug
  char::EscapeDefault
  char::EscapeUnicode
  char::InvalidSequence
  char::ParseCharError
  char::ToLowercase
  char::ToUppercase
  char::UnicodeVersion
  cmp::Reverse
  fmt::Arguments
  fmt::DebugList
  fmt::DebugMap
  fmt::DebugSet
  fmt::DebugStruct
  fmt::DebugTuple
  fmt::Error
  fmt::Formatter
  hash::BuildHasherDefault
  hash::SipHasher
  - iter
    iter::Chain
    iter::Cloned
    iter::Cycle
    iter::Empty
    iter::Enumerate
    iter::Filter
    iter::FilterMap
    iter::FlatMap
    iter::Flatten
    iter::Fuse
    iter::Inspect
    iter::Map
    iter::Once
    iter::Peekable
    iter::Repeat
    iter::RepeatWith
    iter::Rev
    iter::Scan
    iter::Skip
    iter::SkipWhile
    iter::StepBy
    iter::Take
    iter::TakeWhile
    iter::Zip
  marker::PhantomData
  marker::Pinned
  mem::Discriminant
  mem::PinMut
  num::NonZeroU128
  num::NonZeroU16
  num::NonZeroU32
  num::NonZeroU64
  num::NonZeroU8
  num::NonZeroUsize
  num::ParseFloatError
  num::ParseIntError
  num::TryFromIntError
  num::Wrapping
  ops::Range
  ops::RangeFrom
  ops::RangeFull
  ops::RangeInclusive
  ops::RangeTo
  ops::RangeToInclusive
  option::IntoIter
  option::Iter
  option::IterMut
  option::NoneError
  panic::Location
  panic::PanicInfo
  ptr::NonNull
  raw::TraitObject
  result::IntoIter
  result::Iter
  result::IterMut
  - simd
    simd::f32x16
    simd::f32x2
    simd::f32x4
    simd::f32x8
    simd::f64x2
    simd::f64x4
    simd::f64x8
    simd::i16x16
    simd::i16x2
    simd::i16x32
    simd::i16x4
    simd::i16x8
    simd::i32x16
    simd::i32x2
    simd::i32x4
    simd::i32x8
    simd::i64x2
    simd::i64x4
    simd::i64x8
    simd::i8x16
    simd::i8x2
    simd::i8x32
    simd::i8x4
    simd::i8x64
    simd::i8x8
    simd::m16x16
    simd::m16x2
    simd::m16x4
    simd::m16x8
    simd::m1x16
    simd::m1x32
    simd::m1x64
    simd::m1x8
    simd::m32x2
    simd::m32x4
    simd::m32x8
    simd::m64x2
    simd::m64x4
    simd::m8x16
    simd::m8x2
    simd::m8x32
    simd::m8x4
    simd::m8x8
    simd::u16x16
    simd::u16x2
    simd::u16x32
    simd::u16x4
    simd::u16x8
    simd::u32x16
    simd::u32x2
    simd::u32x4
    simd::u32x8
    simd::u64x2
    simd::u64x4
    simd::u64x8
    simd::u8x16
    simd::u8x2
    simd::u8x32
    simd::u8x4
    simd::u8x64
    simd::u8x8
  slice::Chunks
  slice::ChunksMut
  slice::ExactChunks
  slice::ExactChunksMut
  slice::Iter
  slice::IterMut
  slice::RSplit
  slice::RSplitMut
  slice::RSplitN
  slice::RSplitNMut
  slice::Split
  slice::SplitMut
  slice::SplitN
  slice::SplitNMut
  slice::Windows
  str::Bytes
  str::CharIndices
  str::Chars
  str::EncodeUtf16
  str::Lines
  str::LinesAny
  str::MatchIndices
  str::Matches
  str::ParseBoolError
  str::RMatchIndices
  str::RMatches
  str::RSplit
  str::RSplitN
  str::RSplitTerminator
  str::Split
  str::SplitN
  str::SplitTerminator
  str::SplitWhitespace
  str::Utf8Error
  str::lossy::Utf8Lossy
  str::lossy::Utf8LossyChunk
  str::lossy::Utf8LossyChunksIter
  str::pattern::CharPredicateSearcher
  str::pattern::CharSearcher
  str::pattern::CharSliceSearcher
  str::pattern::StrSearcher
  sync::atomic::AtomicBool
  sync::atomic::AtomicI16
  sync::atomic::AtomicI32
  sync::atomic::AtomicI64
  sync::atomic::AtomicI8
  sync::atomic::AtomicIsize
  sync::atomic::AtomicPtr
  sync::atomic::AtomicU16
  sync::atomic::AtomicU32
  sync::atomic::AtomicU64
  sync::atomic::AtomicU8
  sync::atomic::AtomicUsize
  time::Duration

## Enums
  alloc::CollectionAllocErr
  cmp::Ordering
  fmt::Alignment
  num::FpCategory
  ops::Bound
  ops::GeneratorState
  option::Option
  result::Result
  str::pattern::SearchStep
  sync::atomic::Ordering

## Unions
  mem::ManuallyDrop

## Traits
  alloc::Alloc
  alloc::GlobalAlloc
  any::Any
  array::FixedSizeArray
  borrow::Borrow
  borrow::BorrowMut
  clone::Clone
  cmp::Eq
  cmp::Ord
  cmp::PartialEq
  cmp::PartialOrd
  convert::AsMut
  convert::AsRef
  convert::From
  convert::Into
  convert::TryFrom
  convert::TryInto
  default::Default
  fmt::Binary
  fmt::Debug
  fmt::Display
  fmt::LowerExp
  fmt::LowerHex
  fmt::Octal
  fmt::Pointer
  fmt::UpperExp
  fmt::UpperHex
  fmt::Write
  hash::BuildHasher
  hash::Hash
  hash::Hasher
  iter::DoubleEndedIterator
  iter::ExactSizeIterator
  iter::Extend
  iter::FromIterator
  iter::FusedIterator
  iter::IntoIterator
  iter::Iterator
  iter::Product
  iter::Step
  iter::Sum
  iter::TrustedLen
  marker::Copy
  marker::Send
  marker::Sized
  marker::Sync
  marker::Unpin
  marker::Unsize
  ops::Add
  ops::AddAssign
  ops::BitAnd
  ops::BitAndAssign
  ops::BitOr
  ops::BitOrAssign
  ops::BitXor
  ops::BitXorAssign
  ops::CoerceUnsized
  ops::Deref
  ops::DerefMut
  ops::Div
  ops::DivAssign
  ops::Drop
  ops::Fn
  ops::FnMut
  ops::FnOnce
  ops::Generator
  ops::Index
  ops::IndexMut
  ops::Mul
  ops::MulAssign
  ops::Neg
  ops::Not
  ops::RangeBounds
  ops::Rem
  ops::RemAssign
  ops::Shl
  ops::ShlAssign
  ops::Shr
  ops::ShrAssign
  ops::Sub
  ops::SubAssign
  ops::Try
  simd::FromBits
  simd::IntoBits
  slice::SliceIndex
  str::FromStr
  str::pattern::DoubleEndedSearcher
  str::pattern::Pattern
  str::pattern::ReverseSearcher
  str::pattern::Searcher

## Macros
  assert
  assert_eq
  assert_ne
  cfg
  column
  compile_error
  concat
  concat_idents
  debug_assert
  debug_assert_eq
  debug_assert_ne
  env
  file
  format_args
  include
  include_bytes
  include_str
  line
  module_path
  option_env
  panic
  stringify
  try
  unimplemented
  unreachable
  write
  writeln

## Functions
  arch
    arch::aarch64::_cls_u32
    arch::aarch64::_cls_u64
    arch::aarch64::_clz_u64
    arch::aarch64::_rbit_u64
    arch::aarch64::_rev_u16
    arch::aarch64::_rev_u32
    arch::aarch64::_rev_u64
    arch::aarch64::vadd_f32
    arch::aarch64::vadd_f64
    arch::aarch64::vadd_s16
    arch::aarch64::vadd_s32
    arch::aarch64::vadd_s8
    arch::aarch64::vadd_u16
    arch::aarch64::vadd_u32
    arch::aarch64::vadd_u8
    arch::aarch64::vaddd_s64
    arch::aarch64::vaddd_u64
    arch::aarch64::vaddl_s16
    arch::aarch64::vaddl_s32
    arch::aarch64::vaddl_s8
    arch::aarch64::vaddl_u16
    arch::aarch64::vaddl_u32
    arch::aarch64::vaddl_u8
    arch::aarch64::vaddq_f32
    arch::aarch64::vaddq_f64
    arch::aarch64::vaddq_s16
    arch::aarch64::vaddq_s32
    arch::aarch64::vaddq_s64
    arch::aarch64::vaddq_s8
    arch::aarch64::vaddq_u16
    arch::aarch64::vaddq_u32
    arch::aarch64::vaddq_u64
    arch::aarch64::vaddq_u8
    arch::aarch64::vaesdq_u8
    arch::aarch64::vaeseq_u8
    arch::aarch64::vaesimcq_u8
    arch::aarch64::vaesmcq_u8
    arch::aarch64::vmaxv_f32
    arch::aarch64::vmaxv_s16
    arch::aarch64::vmaxv_s32
    arch::aarch64::vmaxv_s8
    arch::aarch64::vmaxv_u16
    arch::aarch64::vmaxv_u32
    arch::aarch64::vmaxv_u8
    arch::aarch64::vmaxvq_f32
    arch::aarch64::vmaxvq_f64
    arch::aarch64::vmaxvq_s16
    arch::aarch64::vmaxvq_s32
    arch::aarch64::vmaxvq_s8
    arch::aarch64::vmaxvq_u16
    arch::aarch64::vmaxvq_u32
    arch::aarch64::vmaxvq_u8
    arch::aarch64::vminv_f32
    arch::aarch64::vminv_s16
    arch::aarch64::vminv_s32
    arch::aarch64::vminv_s8
    arch::aarch64::vminv_u16
    arch::aarch64::vminv_u32
    arch::aarch64::vminv_u8
    arch::aarch64::vminvq_f32
    arch::aarch64::vminvq_f64
    arch::aarch64::vminvq_s16
    arch::aarch64::vminvq_s32
    arch::aarch64::vminvq_s8
    arch::aarch64::vminvq_u16
    arch::aarch64::vminvq_u32
    arch::aarch64::vminvq_u8
    arch::aarch64::vmovl_s16
    arch::aarch64::vmovl_s32
    arch::aarch64::vmovl_s8
    arch::aarch64::vmovl_u16
    arch::aarch64::vmovl_u32
    arch::aarch64::vmovl_u8
    arch::aarch64::vmovn_s16
    arch::aarch64::vmovn_s32
    arch::aarch64::vmovn_s64
    arch::aarch64::vmovn_u16
    arch::aarch64::vmovn_u32
    arch::aarch64::vmovn_u64
    arch::aarch64::vpmax_f32
    arch::aarch64::vpmax_s16
    arch::aarch64::vpmax_s32
    arch::aarch64::vpmax_s8
    arch::aarch64::vpmax_u16
    arch::aarch64::vpmax_u32
    arch::aarch64::vpmax_u8
    arch::aarch64::vpmaxq_f32
    arch::aarch64::vpmaxq_f64
    arch::aarch64::vpmaxq_s16
    arch::aarch64::vpmaxq_s32
    arch::aarch64::vpmaxq_s8
    arch::aarch64::vpmaxq_u16
    arch::aarch64::vpmaxq_u32
    arch::aarch64::vpmaxq_u8
    arch::aarch64::vpmin_f32
    arch::aarch64::vpmin_s16
    arch::aarch64::vpmin_s32
    arch::aarch64::vpmin_s8
    arch::aarch64::vpmin_u16
    arch::aarch64::vpmin_u32
    arch::aarch64::vpmin_u8
    arch::aarch64::vpminq_f32
    arch::aarch64::vpminq_f64
    arch::aarch64::vpminq_s16
    arch::aarch64::vpminq_s32
    arch::aarch64::vpminq_s8
    arch::aarch64::vpminq_u16
    arch::aarch64::vpminq_u32
    arch::aarch64::vpminq_u8
    arch::aarch64::vrsqrte_f32
    arch::aarch64::vsha1cq_u32
    arch::aarch64::vsha1h_u32
    arch::aarch64::vsha1mq_u32
    arch::aarch64::vsha1pq_u32
    arch::aarch64::vsha1su0q_u32
    arch::aarch64::vsha1su1q_u32
    arch::aarch64::vsha256h2q_u32
    arch::aarch64::vsha256hq_u32
    arch::aarch64::vsha256su0q_u32
    arch::aarch64::vsha256su1q_u32
    arch::arm::_rev_u16
    arch::arm::_rev_u32
    arch::arm::vadd_f32
    arch::arm::vadd_s16
    arch::arm::vadd_s32
    arch::arm::vadd_s8
    arch::arm::vadd_u16
    arch::arm::vadd_u32
    arch::arm::vadd_u8
    arch::arm::vaddl_s16
    arch::arm::vaddl_s32
    arch::arm::vaddl_s8
    arch::arm::vaddl_u16
    arch::arm::vaddl_u32
    arch::arm::vaddl_u8
    arch::arm::vaddq_f32
    arch::arm::vaddq_s16
    arch::arm::vaddq_s32
    arch::arm::vaddq_s64
    arch::arm::vaddq_s8
    arch::arm::vaddq_u16
    arch::arm::vaddq_u32
    arch::arm::vaddq_u64
    arch::arm::vaddq_u8
    arch::arm::vmovl_s16
    arch::arm::vmovl_s32
    arch::arm::vmovl_s8
    arch::arm::vmovl_u16
    arch::arm::vmovl_u32
    arch::arm::vmovl_u8
    arch::arm::vmovn_s16
    arch::arm::vmovn_s32
    arch::arm::vmovn_s64
    arch::arm::vmovn_u16
    arch::arm::vmovn_u32
    arch::arm::vmovn_u64
    arch::arm::vpmax_f32
    arch::arm::vpmax_s16
    arch::arm::vpmax_s32
    arch::arm::vpmax_s8
    arch::arm::vpmax_u16
    arch::arm::vpmax_u32
    arch::arm::vpmax_u8
    arch::arm::vpmin_f32
    arch::arm::vpmin_s16
    arch::arm::vpmin_s32
    arch::arm::vpmin_s8
    arch::arm::vpmin_u16
    arch::arm::vpmin_u32
    arch::arm::vpmin_u8
    arch::arm::vrsqrte_f32
    arch::mips64::__msa_add_a_b
    arch::mips::__msa_add_a_b
    arch::x86::_MM_GET_EXCEPTION_MASK
    arch::x86::_MM_GET_EXCEPTION_STATE
    arch::x86::_MM_GET_FLUSH_ZERO_MODE
    arch::x86::_MM_GET_ROUNDING_MODE
    arch::x86::_MM_SET_EXCEPTION_MASK
    arch::x86::_MM_SET_EXCEPTION_STATE
    arch::x86::_MM_SET_FLUSH_ZERO_MODE
    arch::x86::_MM_SET_ROUNDING_MODE
    arch::x86::_MM_TRANSPOSE4_PS
    arch::x86::__cpuid
    arch::x86::__cpuid_count
    arch::x86::__get_cpuid_max
    arch::x86::__rdtscp
    arch::x86::__readeflags
    arch::x86::__writeeflags
    arch::x86::_andn_u32
    arch::x86::_bextr2_u32
    arch::x86::_bextr_u32
    arch::x86::_blcfill_u32
    arch::x86::_blcfill_u64
    arch::x86::_blci_u32
    arch::x86::_blci_u64
    arch::x86::_blcic_u32
    arch::x86::_blcic_u64
    arch::x86::_blcmsk_u32
    arch::x86::_blcmsk_u64
    arch::x86::_blcs_u32
    arch::x86::_blcs_u64
    arch::x86::_blsfill_u32
    arch::x86::_blsfill_u64
    arch::x86::_blsi_u32
    arch::x86::_blsic_u32
    arch::x86::_blsic_u64
    arch::x86::_blsmsk_u32
    arch::x86::_blsr_u32
    arch::x86::_bswap
    arch::x86::_bzhi_u32
    arch::x86::_fxrstor
    arch::x86::_fxsave
    arch::x86::_lzcnt_u32
    arch::x86::_m_maskmovq
    arch::x86::_m_paddb
    arch::x86::_m_paddd
    arch::x86::_m_paddsb
    arch::x86::_m_paddsw
    arch::x86::_m_paddusb
    arch::x86::_m_paddusw
    arch::x86::_m_paddw
    arch::x86::_m_pavgb
    arch::x86::_m_pavgw
    arch::x86::_m_pextrw
    arch::x86::_m_pinsrw
    arch::x86::_m_pmaxsw
    arch::x86::_m_pmaxub
    arch::x86::_m_pminsw
    arch::x86::_m_pminub
    arch::x86::_m_pmovmskb
    arch::x86::_m_pmulhuw
    arch::x86::_m_psadbw
    arch::x86::_m_pshufw
    arch::x86::_m_psubb
    arch::x86::_m_psubd
    arch::x86::_m_psubsb
    arch::x86::_m_psubsw
    arch::x86::_m_psubusb
    arch::x86::_m_psubusw
    arch::x86::_m_psubw
    arch::x86::_mm256_abs_epi16
    arch::x86::_mm256_abs_epi32
    arch::x86::_mm256_abs_epi8
    arch::x86::_mm256_add_epi16
    arch::x86::_mm256_add_epi32
    arch::x86::_mm256_add_epi64
    arch::x86::_mm256_add_epi8
    arch::x86::_mm256_add_pd
    arch::x86::_mm256_add_ps
    arch::x86::_mm256_adds_epi16
    arch::x86::_mm256_adds_epi8
    arch::x86::_mm256_adds_epu16
    arch::x86::_mm256_adds_epu8
    arch::x86::_mm256_addsub_pd
    arch::x86::_mm256_addsub_ps
    arch::x86::_mm256_alignr_epi8
    arch::x86::_mm256_and_pd
    arch::x86::_mm256_and_ps
    arch::x86::_mm256_and_si256
    arch::x86::_mm256_andnot_pd
    arch::x86::_mm256_andnot_ps
    arch::x86::_mm256_andnot_si256
    arch::x86::_mm256_avg_epu16
    arch::x86::_mm256_avg_epu8
    arch::x86::_mm256_blend_epi16
    arch::x86::_mm256_blend_epi32
    arch::x86::_mm256_blend_pd
    arch::x86::_mm256_blend_ps
    arch::x86::_mm256_blendv_epi8
    arch::x86::_mm256_blendv_pd
    arch::x86::_mm256_blendv_ps
    arch::x86::_mm256_broadcast_pd
    arch::x86::_mm256_broadcast_ps
    arch::x86::_mm256_broadcast_sd
    arch::x86::_mm256_broadcast_ss
    arch::x86::_mm256_broadcastb_epi8
    arch::x86::_mm256_broadcastd_epi32
    arch::x86::_mm256_broadcastq_epi64
    arch::x86::_mm256_broadcastsd_pd
    arch::x86::_mm256_broadcastsi128_si256
    arch::x86::_mm256_broadcastss_ps
    arch::x86::_mm256_broadcastw_epi16
    arch::x86::_mm256_bslli_epi128
    arch::x86::_mm256_bsrli_epi128
    arch::x86::_mm256_castpd128_pd256
    arch::x86::_mm256_castpd256_pd128
    arch::x86::_mm256_castpd_ps
    arch::x86::_mm256_castpd_si256
    arch::x86::_mm256_castps128_ps256
    arch::x86::_mm256_castps256_ps128
    arch::x86::_mm256_castps_pd
    arch::x86::_mm256_castps_si256
    arch::x86::_mm256_castsi128_si256
    arch::x86::_mm256_castsi256_pd
    arch::x86::_mm256_castsi256_ps
    arch::x86::_mm256_castsi256_si128
    arch::x86::_mm256_ceil_pd
    arch::x86::_mm256_ceil_ps
    arch::x86::_mm256_cmp_pd
    arch::x86::_mm256_cmp_ps
    arch::x86::_mm256_cmpeq_epi16
    arch::x86::_mm256_cmpeq_epi32
    arch::x86::_mm256_cmpeq_epi64
    arch::x86::_mm256_cmpeq_epi8
    arch::x86::_mm256_cmpgt_epi16
    arch::x86::_mm256_cmpgt_epi32
    arch::x86::_mm256_cmpgt_epi64
    arch::x86::_mm256_cmpgt_epi8
    arch::x86::_mm256_cvtepi16_epi32
    arch::x86::_mm256_cvtepi16_epi64
    arch::x86::_mm256_cvtepi32_epi64
    arch::x86::_mm256_cvtepi32_pd
    arch::x86::_mm256_cvtepi32_ps
    arch::x86::_mm256_cvtepi8_epi16
    arch::x86::_mm256_cvtepi8_epi32
    arch::x86::_mm256_cvtepi8_epi64
    arch::x86::_mm256_cvtepu16_epi32
    arch::x86::_mm256_cvtepu16_epi64
    arch::x86::_mm256_cvtepu32_epi64
    arch::x86::_mm256_cvtepu8_epi16
    arch::x86::_mm256_cvtepu8_epi32
    arch::x86::_mm256_cvtepu8_epi64
    arch::x86::_mm256_cvtpd_epi32
    arch::x86::_mm256_cvtpd_ps
    arch::x86::_mm256_cvtps_epi32
    arch::x86::_mm256_cvtps_pd
    arch::x86::_mm256_cvtsd_f64
    arch::x86::_mm256_cvtsi256_si32
    arch::x86::_mm256_cvtss_f32
    arch::x86::_mm256_cvttpd_epi32
    arch::x86::_mm256_cvttps_epi32
    arch::x86::_mm256_div_pd
    arch::x86::_mm256_div_ps
    arch::x86::_mm256_dp_ps
    arch::x86::_mm256_extract_epi16
    arch::x86::_mm256_extract_epi32
    arch::x86::_mm256_extract_epi8
    arch::x86::_mm256_extractf128_pd
    arch::x86::_mm256_extractf128_ps
    arch::x86::_mm256_extractf128_si256
    arch::x86::_mm256_extracti128_si256
    arch::x86::_mm256_floor_pd
    arch::x86::_mm256_floor_ps
    arch::x86::_mm256_fmadd_pd
    arch::x86::_mm256_fmadd_ps
    arch::x86::_mm256_fmaddsub_pd
    arch::x86::_mm256_fmaddsub_ps
    arch::x86::_mm256_fmsub_pd
    arch::x86::_mm256_fmsub_ps
    arch::x86::_mm256_fmsubadd_pd
    arch::x86::_mm256_fmsubadd_ps
    arch::x86::_mm256_fnmadd_pd
    arch::x86::_mm256_fnmadd_ps
    arch::x86::_mm256_fnmsub_pd
    arch::x86::_mm256_fnmsub_ps
    arch::x86::_mm256_hadd_epi16
    arch::x86::_mm256_hadd_epi32
    arch::x86::_mm256_hadd_pd
    arch::x86::_mm256_hadd_ps
    arch::x86::_mm256_hadds_epi16
    arch::x86::_mm256_hsub_epi16
    arch::x86::_mm256_hsub_epi32
    arch::x86::_mm256_hsub_pd
    arch::x86::_mm256_hsub_ps
    arch::x86::_mm256_hsubs_epi16
    arch::x86::_mm256_i32gather_epi32
    arch::x86::_mm256_i32gather_epi64
    arch::x86::_mm256_i32gather_pd
    arch::x86::_mm256_i32gather_ps
    arch::x86::_mm256_i64gather_epi32
    arch::x86::_mm256_i64gather_epi64
    arch::x86::_mm256_i64gather_pd
    arch::x86::_mm256_i64gather_ps
    arch::x86::_mm256_insert_epi16
    arch::x86::_mm256_insert_epi32
    arch::x86::_mm256_insert_epi8
    arch::x86::_mm256_insertf128_pd
    arch::x86::_mm256_insertf128_ps
    arch::x86::_mm256_insertf128_si256
    arch::x86::_mm256_inserti128_si256
    arch::x86::_mm256_lddqu_si256
    arch::x86::_mm256_load_pd
    arch::x86::_mm256_load_ps
    arch::x86::_mm256_load_si256
    arch::x86::_mm256_loadu2_m128
    arch::x86::_mm256_loadu2_m128d
    arch::x86::_mm256_loadu2_m128i
    arch::x86::_mm256_loadu_pd
    arch::x86::_mm256_loadu_ps
    arch::x86::_mm256_loadu_si256
    arch::x86::_mm256_madd_epi16
    arch::x86::_mm256_maddubs_epi16
    arch::x86::_mm256_mask_i32gather_epi32
    arch::x86::_mm256_mask_i32gather_epi64
    arch::x86::_mm256_mask_i32gather_pd
    arch::x86::_mm256_mask_i32gather_ps
    arch::x86::_mm256_mask_i64gather_epi32
    arch::x86::_mm256_mask_i64gather_epi64
    arch::x86::_mm256_mask_i64gather_pd
    arch::x86::_mm256_mask_i64gather_ps
    arch::x86::_mm256_maskload_epi32
    arch::x86::_mm256_maskload_epi64
    arch::x86::_mm256_maskload_pd
    arch::x86::_mm256_maskload_ps
    arch::x86::_mm256_maskstore_epi32
    arch::x86::_mm256_maskstore_epi64
    arch::x86::_mm256_maskstore_pd
    arch::x86::_mm256_maskstore_ps
    arch::x86::_mm256_max_epi16
    arch::x86::_mm256_max_epi32
    arch::x86::_mm256_max_epi8
    arch::x86::_mm256_max_epu16
    arch::x86::_mm256_max_epu32
    arch::x86::_mm256_max_epu8
    arch::x86::_mm256_max_pd
    arch::x86::_mm256_max_ps
    arch::x86::_mm256_min_epi16
    arch::x86::_mm256_min_epi32
    arch::x86::_mm256_min_epi8
    arch::x86::_mm256_min_epu16
    arch::x86::_mm256_min_epu32
    arch::x86::_mm256_min_epu8
    arch::x86::_mm256_min_pd
    arch::x86::_mm256_min_ps
    arch::x86::_mm256_movedup_pd
    arch::x86::_mm256_movehdup_ps
    arch::x86::_mm256_moveldup_ps
    arch::x86::_mm256_movemask_epi8
    arch::x86::_mm256_movemask_pd
    arch::x86::_mm256_movemask_ps
    arch::x86::_mm256_mpsadbw_epu8
    arch::x86::_mm256_mul_epi32
    arch::x86::_mm256_mul_epu32
    arch::x86::_mm256_mul_pd
    arch::x86::_mm256_mul_ps
    arch::x86::_mm256_mulhi_epi16
    arch::x86::_mm256_mulhi_epu16
    arch::x86::_mm256_mulhrs_epi16
    arch::x86::_mm256_mullo_epi16
    arch::x86::_mm256_mullo_epi32
    arch::x86::_mm256_or_pd
    arch::x86::_mm256_or_ps
    arch::x86::_mm256_or_si256
    arch::x86::_mm256_packs_epi16
    arch::x86::_mm256_packs_epi32
    arch::x86::_mm256_packus_epi16
    arch::x86::_mm256_packus_epi32
    arch::x86::_mm256_permute2f128_pd
    arch::x86::_mm256_permute2f128_ps
    arch::x86::_mm256_permute2f128_si256
    arch::x86::_mm256_permute2x128_si256
    arch::x86::_mm256_permute4x64_epi64
    arch::x86::_mm256_permute4x64_pd
    arch::x86::_mm256_permute_pd
    arch::x86::_mm256_permute_ps
    arch::x86::_mm256_permutevar8x32_epi32
    arch::x86::_mm256_permutevar8x32_ps
    arch::x86::_mm256_permutevar_pd
    arch::x86::_mm256_permutevar_ps
    arch::x86::_mm256_rcp_ps
    arch::x86::_mm256_round_pd
    arch::x86::_mm256_round_ps
    arch::x86::_mm256_rsqrt_ps
    arch::x86::_mm256_sad_epu8
    arch::x86::_mm256_set1_epi16
    arch::x86::_mm256_set1_epi32
    arch::x86::_mm256_set1_epi64x
    arch::x86::_mm256_set1_epi8
    arch::x86::_mm256_set1_pd
    arch::x86::_mm256_set1_ps
    arch::x86::_mm256_set_epi16
    arch::x86::_mm256_set_epi32
    arch::x86::_mm256_set_epi64x
    arch::x86::_mm256_set_epi8
    arch::x86::_mm256_set_m128
    arch::x86::_mm256_set_m128d
    arch::x86::_mm256_set_m128i
    arch::x86::_mm256_set_pd
    arch::x86::_mm256_set_ps
    arch::x86::_mm256_setr_epi16
    arch::x86::_mm256_setr_epi32
    arch::x86::_mm256_setr_epi64x
    arch::x86::_mm256_setr_epi8
    arch::x86::_mm256_setr_m128
    arch::x86::_mm256_setr_m128d
    arch::x86::_mm256_setr_m128i
    arch::x86::_mm256_setr_pd
    arch::x86::_mm256_setr_ps
    arch::x86::_mm256_setzero_pd
    arch::x86::_mm256_setzero_ps
    arch::x86::_mm256_setzero_si256
    arch::x86::_mm256_shuffle_epi32
    arch::x86::_mm256_shuffle_epi8
    arch::x86::_mm256_shuffle_pd
    arch::x86::_mm256_shuffle_ps
    arch::x86::_mm256_shufflehi_epi16
    arch::x86::_mm256_shufflelo_epi16
    arch::x86::_mm256_sign_epi16
    arch::x86::_mm256_sign_epi32
    arch::x86::_mm256_sign_epi8
    arch::x86::_mm256_sll_epi16
    arch::x86::_mm256_sll_epi32
    arch::x86::_mm256_sll_epi64
    arch::x86::_mm256_slli_epi16
    arch::x86::_mm256_slli_epi32
    arch::x86::_mm256_slli_epi64
    arch::x86::_mm256_slli_si256
    arch::x86::_mm256_sllv_epi32
    arch::x86::_mm256_sllv_epi64
    arch::x86::_mm256_sqrt_pd
    arch::x86::_mm256_sqrt_ps
    arch::x86::_mm256_sra_epi16
    arch::x86::_mm256_sra_epi32
    arch::x86::_mm256_srai_epi16
    arch::x86::_mm256_srai_epi32
    arch::x86::_mm256_srav_epi32
    arch::x86::_mm256_srl_epi16
    arch::x86::_mm256_srl_epi32
    arch::x86::_mm256_srl_epi64
    arch::x86::_mm256_srli_epi16
    arch::x86::_mm256_srli_epi32
    arch::x86::_mm256_srli_epi64
    arch::x86::_mm256_srli_si256
    arch::x86::_mm256_srlv_epi32
    arch::x86::_mm256_srlv_epi64
    arch::x86::_mm256_store_pd
    arch::x86::_mm256_store_ps
    arch::x86::_mm256_store_si256
    arch::x86::_mm256_storeu2_m128
    arch::x86::_mm256_storeu2_m128d
    arch::x86::_mm256_storeu2_m128i
    arch::x86::_mm256_storeu_pd
    arch::x86::_mm256_storeu_ps
    arch::x86::_mm256_storeu_si256
    arch::x86::_mm256_stream_pd
    arch::x86::_mm256_stream_ps
    arch::x86::_mm256_stream_si256
    arch::x86::_mm256_sub_epi16
    arch::x86::_mm256_sub_epi32
    arch::x86::_mm256_sub_epi64
    arch::x86::_mm256_sub_epi8
    arch::x86::_mm256_sub_pd
    arch::x86::_mm256_sub_ps
    arch::x86::_mm256_subs_epi16
    arch::x86::_mm256_subs_epi8
    arch::x86::_mm256_subs_epu16
    arch::x86::_mm256_subs_epu8
    arch::x86::_mm256_testc_pd
    arch::x86::_mm256_testc_ps
    arch::x86::_mm256_testc_si256
    arch::x86::_mm256_testnzc_pd
    arch::x86::_mm256_testnzc_ps
    arch::x86::_mm256_testnzc_si256
    arch::x86::_mm256_testz_pd
    arch::x86::_mm256_testz_ps
    arch::x86::_mm256_testz_si256
    arch::x86::_mm256_undefined_pd
    arch::x86::_mm256_undefined_ps
    arch::x86::_mm256_undefined_si256
    arch::x86::_mm256_unpackhi_epi16
    arch::x86::_mm256_unpackhi_epi32
    arch::x86::_mm256_unpackhi_epi64
    arch::x86::_mm256_unpackhi_epi8
    arch::x86::_mm256_unpackhi_pd
    arch::x86::_mm256_unpackhi_ps
    arch::x86::_mm256_unpacklo_epi16
    arch::x86::_mm256_unpacklo_epi32
    arch::x86::_mm256_unpacklo_epi64
    arch::x86::_mm256_unpacklo_epi8
    arch::x86::_mm256_unpacklo_pd
    arch::x86::_mm256_unpacklo_ps
    arch::x86::_mm256_xor_pd
    arch::x86::_mm256_xor_ps
    arch::x86::_mm256_xor_si256
    arch::x86::_mm256_zeroall
    arch::x86::_mm256_zeroupper
    arch::x86::_mm256_zextpd128_pd256
    arch::x86::_mm256_zextps128_ps256
    arch::x86::_mm256_zextsi128_si256
    arch::x86::_mm_abs_epi16
    arch::x86::_mm_abs_epi32
    arch::x86::_mm_abs_epi8
    arch::x86::_mm_abs_pi16
    arch::x86::_mm_abs_pi32
    arch::x86::_mm_abs_pi8
    arch::x86::_mm_add_epi16
    arch::x86::_mm_add_epi32
    arch::x86::_mm_add_epi64
    arch::x86::_mm_add_epi8
    arch::x86::_mm_add_pd
    arch::x86::_mm_add_pi16
    arch::x86::_mm_add_pi32
    arch::x86::_mm_add_pi8
    arch::x86::_mm_add_ps
    arch::x86::_mm_add_sd
    arch::x86::_mm_add_si64
    arch::x86::_mm_add_ss
    arch::x86::_mm_adds_epi16
    arch::x86::_mm_adds_epi8
    arch::x86::_mm_adds_epu16
    arch::x86::_mm_adds_epu8
    arch::x86::_mm_adds_pi16
    arch::x86::_mm_adds_pi8
    arch::x86::_mm_adds_pu16
    arch::x86::_mm_adds_pu8
    arch::x86::_mm_addsub_pd
    arch::x86::_mm_addsub_ps
    arch::x86::_mm_aesdec_si128
    arch::x86::_mm_aesdeclast_si128
    arch::x86::_mm_aesenc_si128
    arch::x86::_mm_aesenclast_si128
    arch::x86::_mm_aesimc_si128
    arch::x86::_mm_aeskeygenassist_si128
    arch::x86::_mm_alignr_epi8
    arch::x86::_mm_alignr_pi8
    arch::x86::_mm_and_pd
    arch::x86::_mm_and_ps
    arch::x86::_mm_and_si128
    arch::x86::_mm_andnot_pd
    arch::x86::_mm_andnot_ps
    arch::x86::_mm_andnot_si128
    arch::x86::_mm_avg_epu16
    arch::x86::_mm_avg_epu8
    arch::x86::_mm_avg_pu16
    arch::x86::_mm_avg_pu8
    arch::x86::_mm_blend_epi16
    arch::x86::_mm_blend_epi32
    arch::x86::_mm_blend_pd
    arch::x86::_mm_blend_ps
    arch::x86::_mm_blendv_epi8
    arch::x86::_mm_blendv_pd
    arch::x86::_mm_blendv_ps
    arch::x86::_mm_broadcast_ss
    arch::x86::_mm_broadcastb_epi8
    arch::x86::_mm_broadcastd_epi32
    arch::x86::_mm_broadcastq_epi64
    arch::x86::_mm_broadcastsd_pd
    arch::x86::_mm_broadcastss_ps
    arch::x86::_mm_broadcastw_epi16
    arch::x86::_mm_bslli_si128
    arch::x86::_mm_bsrli_si128
    arch::x86::_mm_castpd_ps
    arch::x86::_mm_castpd_si128
    arch::x86::_mm_castps_pd
    arch::x86::_mm_castps_si128
    arch::x86::_mm_castsi128_pd
    arch::x86::_mm_castsi128_ps
    arch::x86::_mm_ceil_pd
    arch::x86::_mm_ceil_ps
    arch::x86::_mm_ceil_sd
    arch::x86::_mm_ceil_ss
    arch::x86::_mm_clflush
    arch::x86::_mm_clmulepi64_si128
    arch::x86::_mm_cmp_pd
    arch::x86::_mm_cmp_ps
    arch::x86::_mm_cmp_sd
    arch::x86::_mm_cmp_ss
    arch::x86::_mm_cmpeq_epi16
    arch::x86::_mm_cmpeq_epi32
    arch::x86::_mm_cmpeq_epi64
    arch::x86::_mm_cmpeq_epi8
    arch::x86::_mm_cmpeq_pd
    arch::x86::_mm_cmpeq_ps
    arch::x86::_mm_cmpeq_sd
    arch::x86::_mm_cmpeq_ss
    arch::x86::_mm_cmpestra
    arch::x86::_mm_cmpestrc
    arch::x86::_mm_cmpestri
    arch::x86::_mm_cmpestrm
    arch::x86::_mm_cmpestro
    arch::x86::_mm_cmpestrs
    arch::x86::_mm_cmpestrz
    arch::x86::_mm_cmpge_pd
    arch::x86::_mm_cmpge_ps
    arch::x86::_mm_cmpge_sd
    arch::x86::_mm_cmpge_ss
    arch::x86::_mm_cmpgt_epi16
    arch::x86::_mm_cmpgt_epi32
    arch::x86::_mm_cmpgt_epi64
    arch::x86::_mm_cmpgt_epi8
    arch::x86::_mm_cmpgt_pd
    arch::x86::_mm_cmpgt_pi16
    arch::x86::_mm_cmpgt_pi32
    arch::x86::_mm_cmpgt_pi8
    arch::x86::_mm_cmpgt_ps
    arch::x86::_mm_cmpgt_sd
    arch::x86::_mm_cmpgt_ss
    arch::x86::_mm_cmpistra
    arch::x86::_mm_cmpistrc
    arch::x86::_mm_cmpistri
    arch::x86::_mm_cmpistrm
    arch::x86::_mm_cmpistro
    arch::x86::_mm_cmpistrs
    arch::x86::_mm_cmpistrz
    arch::x86::_mm_cmple_pd
    arch::x86::_mm_cmple_ps
    arch::x86::_mm_cmple_sd
    arch::x86::_mm_cmple_ss
    arch::x86::_mm_cmplt_epi16
    arch::x86::_mm_cmplt_epi32
    arch::x86::_mm_cmplt_epi8
    arch::x86::_mm_cmplt_pd
    arch::x86::_mm_cmplt_ps
    arch::x86::_mm_cmplt_sd
    arch::x86::_mm_cmplt_ss
    arch::x86::_mm_cmpneq_pd
    arch::x86::_mm_cmpneq_ps
    arch::x86::_mm_cmpneq_sd
    arch::x86::_mm_cmpneq_ss
    arch::x86::_mm_cmpnge_pd
    arch::x86::_mm_cmpnge_ps
    arch::x86::_mm_cmpnge_sd
    arch::x86::_mm_cmpnge_ss
    arch::x86::_mm_cmpngt_pd
    arch::x86::_mm_cmpngt_ps
    arch::x86::_mm_cmpngt_sd
    arch::x86::_mm_cmpngt_ss
    arch::x86::_mm_cmpnle_pd
    arch::x86::_mm_cmpnle_ps
    arch::x86::_mm_cmpnle_sd
    arch::x86::_mm_cmpnle_ss
    arch::x86::_mm_cmpnlt_pd
    arch::x86::_mm_cmpnlt_ps
    arch::x86::_mm_cmpnlt_sd
    arch::x86::_mm_cmpnlt_ss
    arch::x86::_mm_cmpord_pd
    arch::x86::_mm_cmpord_ps
    arch::x86::_mm_cmpord_sd
    arch::x86::_mm_cmpord_ss
    arch::x86::_mm_cmpunord_pd
    arch::x86::_mm_cmpunord_ps
    arch::x86::_mm_cmpunord_sd
    arch::x86::_mm_cmpunord_ss
    arch::x86::_mm_comieq_sd
    arch::x86::_mm_comieq_ss
    arch::x86::_mm_comige_sd
    arch::x86::_mm_comige_ss
    arch::x86::_mm_comigt_sd
    arch::x86::_mm_comigt_ss
    arch::x86::_mm_comile_sd
    arch::x86::_mm_comile_ss
    arch::x86::_mm_comilt_sd
    arch::x86::_mm_comilt_ss
    arch::x86::_mm_comineq_sd
    arch::x86::_mm_comineq_ss
    arch::x86::_mm_crc32_u16
    arch::x86::_mm_crc32_u32
    arch::x86::_mm_crc32_u8
    arch::x86::_mm_cvt_pi2ps
    arch::x86::_mm_cvt_ps2pi
    arch::x86::_mm_cvt_si2ss
    arch::x86::_mm_cvt_ss2si
    arch::x86::_mm_cvtepi16_epi32
    arch::x86::_mm_cvtepi16_epi64
    arch::x86::_mm_cvtepi32_epi64
    arch::x86::_mm_cvtepi32_pd
    arch::x86::_mm_cvtepi32_ps
    arch::x86::_mm_cvtepi8_epi16
    arch::x86::_mm_cvtepi8_epi32
    arch::x86::_mm_cvtepi8_epi64
    arch::x86::_mm_cvtepu16_epi32
    arch::x86::_mm_cvtepu16_epi64
    arch::x86::_mm_cvtepu32_epi64
    arch::x86::_mm_cvtepu8_epi16
    arch::x86::_mm_cvtepu8_epi32
    arch::x86::_mm_cvtepu8_epi64
    arch::x86::_mm_cvtpd_epi32
    arch::x86::_mm_cvtpd_pi32
    arch::x86::_mm_cvtpd_ps
    arch::x86::_mm_cvtpi16_ps
    arch::x86::_mm_cvtpi32_pd
    arch::x86::_mm_cvtpi32_ps
    arch::x86::_mm_cvtpi32x2_ps
    arch::x86::_mm_cvtpi8_ps
    arch::x86::_mm_cvtps_epi32
    arch::x86::_mm_cvtps_pd
    arch::x86::_mm_cvtps_pi16
    arch::x86::_mm_cvtps_pi32
    arch::x86::_mm_cvtps_pi8
    arch::x86::_mm_cvtpu16_ps
    arch::x86::_mm_cvtpu8_ps
    arch::x86::_mm_cvtsd_f64
    arch::x86::_mm_cvtsd_si32
    arch::x86::_mm_cvtsd_ss
    arch::x86::_mm_cvtsi128_si32
    arch::x86::_mm_cvtsi32_sd
    arch::x86::_mm_cvtsi32_si128
    arch::x86::_mm_cvtsi32_ss
    arch::x86::_mm_cvtss_f32
    arch::x86::_mm_cvtss_sd
    arch::x86::_mm_cvtss_si32
    arch::x86::_mm_cvtt_ps2pi
    arch::x86::_mm_cvtt_ss2si
    arch::x86::_mm_cvttpd_epi32
    arch::x86::_mm_cvttpd_pi32
    arch::x86::_mm_cvttps_epi32
    arch::x86::_mm_cvttps_pi32
    arch::x86::_mm_cvttsd_si32
    arch::x86::_mm_cvttss_si32
    arch::x86::_mm_div_pd
    arch::x86::_mm_div_ps
    arch::x86::_mm_div_sd
    arch::x86::_mm_div_ss
    arch::x86::_mm_dp_pd
    arch::x86::_mm_dp_ps
    arch::x86::_mm_extract_epi16
    arch::x86::_mm_extract_epi32
    arch::x86::_mm_extract_epi8
    arch::x86::_mm_extract_pi16
    arch::x86::_mm_extract_ps
    arch::x86::_mm_extract_si64
    arch::x86::_mm_floor_pd
    arch::x86::_mm_floor_ps
    arch::x86::_mm_floor_sd
    arch::x86::_mm_floor_ss
    arch::x86::_mm_fmadd_pd
    arch::x86::_mm_fmadd_ps
    arch::x86::_mm_fmadd_sd
    arch::x86::_mm_fmadd_ss
    arch::x86::_mm_fmaddsub_pd
    arch::x86::_mm_fmaddsub_ps
    arch::x86::_mm_fmsub_pd
    arch::x86::_mm_fmsub_ps
    arch::x86::_mm_fmsub_sd
    arch::x86::_mm_fmsub_ss
    arch::x86::_mm_fmsubadd_pd
    arch::x86::_mm_fmsubadd_ps
    arch::x86::_mm_fnmadd_pd
    arch::x86::_mm_fnmadd_ps
    arch::x86::_mm_fnmadd_sd
    arch::x86::_mm_fnmadd_ss
    arch::x86::_mm_fnmsub_pd
    arch::x86::_mm_fnmsub_ps
    arch::x86::_mm_fnmsub_sd
    arch::x86::_mm_fnmsub_ss
    arch::x86::_mm_getcsr
    arch::x86::_mm_hadd_epi16
    arch::x86::_mm_hadd_epi32
    arch::x86::_mm_hadd_pd
    arch::x86::_mm_hadd_pi16
    arch::x86::_mm_hadd_pi32
    arch::x86::_mm_hadd_ps
    arch::x86::_mm_hadds_epi16
    arch::x86::_mm_hadds_pi16
    arch::x86::_mm_hsub_epi16
    arch::x86::_mm_hsub_epi32
    arch::x86::_mm_hsub_pd
    arch::x86::_mm_hsub_pi16
    arch::x86::_mm_hsub_pi32
    arch::x86::_mm_hsub_ps
    arch::x86::_mm_hsubs_epi16
    arch::x86::_mm_hsubs_pi16
    arch::x86::_mm_i32gather_epi32
    arch::x86::_mm_i32gather_epi64
    arch::x86::_mm_i32gather_pd
    arch::x86::_mm_i32gather_ps
    arch::x86::_mm_i64gather_epi32
    arch::x86::_mm_i64gather_epi64
    arch::x86::_mm_i64gather_pd
    arch::x86::_mm_i64gather_ps
    arch::x86::_mm_insert_epi16
    arch::x86::_mm_insert_epi32
    arch::x86::_mm_insert_epi8
    arch::x86::_mm_insert_pi16
    arch::x86::_mm_insert_ps
    arch::x86::_mm_insert_si64
    arch::x86::_mm_lddqu_si128
    arch::x86::_mm_lfence
    arch::x86::_mm_load1_pd
    arch::x86::_mm_load1_ps
    arch::x86::_mm_load_pd
    arch::x86::_mm_load_pd1
    arch::x86::_mm_load_ps
    arch::x86::_mm_load_ps1
    arch::x86::_mm_load_sd
    arch::x86::_mm_load_si128
    arch::x86::_mm_load_ss
    arch::x86::_mm_loaddup_pd
    arch::x86::_mm_loadh_pd
    arch::x86::_mm_loadh_pi
    arch::x86::_mm_loadl_epi64
    arch::x86::_mm_loadl_pd
    arch::x86::_mm_loadl_pi
    arch::x86::_mm_loadr_pd
    arch::x86::_mm_loadr_ps
    arch::x86::_mm_loadu_pd
    arch::x86::_mm_loadu_ps
    arch::x86::_mm_loadu_si128
    arch::x86::_mm_madd_epi16
    arch::x86::_mm_maddubs_epi16
    arch::x86::_mm_maddubs_pi16
    arch::x86::_mm_mask_i32gather_epi32
    arch::x86::_mm_mask_i32gather_epi64
    arch::x86::_mm_mask_i32gather_pd
    arch::x86::_mm_mask_i32gather_ps
    arch::x86::_mm_mask_i64gather_epi32
    arch::x86::_mm_mask_i64gather_epi64
    arch::x86::_mm_mask_i64gather_pd
    arch::x86::_mm_mask_i64gather_ps
    arch::x86::_mm_maskload_epi32
    arch::x86::_mm_maskload_epi64
    arch::x86::_mm_maskload_pd
    arch::x86::_mm_maskload_ps
    arch::x86::_mm_maskmove_si64
    arch::x86::_mm_maskmoveu_si128
    arch::x86::_mm_maskstore_epi32
    arch::x86::_mm_maskstore_epi64
    arch::x86::_mm_maskstore_pd
    arch::x86::_mm_maskstore_ps
    arch::x86::_mm_max_epi16
    arch::x86::_mm_max_epi32
    arch::x86::_mm_max_epi8
    arch::x86::_mm_max_epu16
    arch::x86::_mm_max_epu32
    arch::x86::_mm_max_epu8
    arch::x86::_mm_max_pd
    arch::x86::_mm_max_pi16
    arch::x86::_mm_max_ps
    arch::x86::_mm_max_pu8
    arch::x86::_mm_max_sd
    arch::x86::_mm_max_ss
    arch::x86::_mm_mfence
    arch::x86::_mm_min_epi16
    arch::x86::_mm_min_epi32
    arch::x86::_mm_min_epi8
    arch::x86::_mm_min_epu16
    arch::x86::_mm_min_epu32
    arch::x86::_mm_min_epu8
    arch::x86::_mm_min_pd
    arch::x86::_mm_min_pi16
    arch::x86::_mm_min_ps
    arch::x86::_mm_min_pu8
    arch::x86::_mm_min_sd
    arch::x86::_mm_min_ss
    arch::x86::_mm_minpos_epu16
    arch::x86::_mm_move_epi64
    arch::x86::_mm_move_sd
    arch::x86::_mm_move_ss
    arch::x86::_mm_movedup_pd
    arch::x86::_mm_movehdup_ps
    arch::x86::_mm_movehl_ps
    arch::x86::_mm_moveldup_ps
    arch::x86::_mm_movelh_ps
    arch::x86::_mm_movemask_epi8
    arch::x86::_mm_movemask_pd
    arch::x86::_mm_movemask_pi8
    arch::x86::_mm_movemask_ps
    arch::x86::_mm_movepi64_pi64
    arch::x86::_mm_movpi64_epi64
    arch::x86::_mm_mpsadbw_epu8
    arch::x86::_mm_mul_epi32
    arch::x86::_mm_mul_epu32
    arch::x86::_mm_mul_pd
    arch::x86::_mm_mul_ps
    arch::x86::_mm_mul_sd
    arch::x86::_mm_mul_ss
    arch::x86::_mm_mul_su32
    arch::x86::_mm_mulhi_epi16
    arch::x86::_mm_mulhi_epu16
    arch::x86::_mm_mulhi_pu16
    arch::x86::_mm_mulhrs_epi16
    arch::x86::_mm_mulhrs_pi16
    arch::x86::_mm_mullo_epi16
    arch::x86::_mm_mullo_epi32
    arch::x86::_mm_or_pd
    arch::x86::_mm_or_ps
    arch::x86::_mm_or_si128
    arch::x86::_mm_packs_epi16
    arch::x86::_mm_packs_epi32
    arch::x86::_mm_packs_pi16
    arch::x86::_mm_packs_pi32
    arch::x86::_mm_packus_epi16
    arch::x86::_mm_packus_epi32
    arch::x86::_mm_pause
    arch::x86::_mm_permute_pd
    arch::x86::_mm_permute_ps
    arch::x86::_mm_permutevar_pd
    arch::x86::_mm_permutevar_ps
    arch::x86::_mm_prefetch
    arch::x86::_mm_rcp_ps
    arch::x86::_mm_rcp_ss
    arch::x86::_mm_round_pd
    arch::x86::_mm_round_ps
    arch::x86::_mm_round_sd
    arch::x86::_mm_round_ss
    arch::x86::_mm_rsqrt_ps
    arch::x86::_mm_rsqrt_ss
    arch::x86::_mm_sad_epu8
    arch::x86::_mm_sad_pu8
    arch::x86::_mm_set1_epi16
    arch::x86::_mm_set1_epi32
    arch::x86::_mm_set1_epi64
    arch::x86::_mm_set1_epi64x
    arch::x86::_mm_set1_epi8
    arch::x86::_mm_set1_pd
    arch::x86::_mm_set1_pi16
    arch::x86::_mm_set1_pi32
    arch::x86::_mm_set1_pi8
    arch::x86::_mm_set1_ps
    arch::x86::_mm_set_epi16
    arch::x86::_mm_set_epi32
    arch::x86::_mm_set_epi64
    arch::x86::_mm_set_epi64x
    arch::x86::_mm_set_epi8
    arch::x86::_mm_set_pd
    arch::x86::_mm_set_pd1
    arch::x86::_mm_set_pi16
    arch::x86::_mm_set_pi32
    arch::x86::_mm_set_pi8
    arch::x86::_mm_set_ps
    arch::x86::_mm_set_ps1
    arch::x86::_mm_set_sd
    arch::x86::_mm_set_ss
    arch::x86::_mm_setcsr
    arch::x86::_mm_setr_epi16
    arch::x86::_mm_setr_epi32
    arch::x86::_mm_setr_epi64
    arch::x86::_mm_setr_epi8
    arch::x86::_mm_setr_pd
    arch::x86::_mm_setr_pi16
    arch::x86::_mm_setr_pi32
    arch::x86::_mm_setr_pi8
    arch::x86::_mm_setr_ps
    arch::x86::_mm_setzero_pd
    arch::x86::_mm_setzero_ps
    arch::x86::_mm_setzero_si128
    arch::x86::_mm_setzero_si64
    arch::x86::_mm_sfence
    arch::x86::_mm_sha1msg1_epu32
    arch::x86::_mm_sha1msg2_epu32
    arch::x86::_mm_sha1nexte_epu32
    arch::x86::_mm_sha1rnds4_epu32
    arch::x86::_mm_sha256msg1_epu32
    arch::x86::_mm_sha256msg2_epu32
    arch::x86::_mm_sha256rnds2_epu32
    arch::x86::_mm_shuffle_epi32
    arch::x86::_mm_shuffle_epi8
    arch::x86::_mm_shuffle_pd
    arch::x86::_mm_shuffle_pi16
    arch::x86::_mm_shuffle_pi8
    arch::x86::_mm_shuffle_ps
    arch::x86::_mm_shufflehi_epi16
    arch::x86::_mm_shufflelo_epi16
    arch::x86::_mm_sign_epi16
    arch::x86::_mm_sign_epi32
    arch::x86::_mm_sign_epi8
    arch::x86::_mm_sign_pi16
    arch::x86::_mm_sign_pi32
    arch::x86::_mm_sign_pi8
    arch::x86::_mm_sll_epi16
    arch::x86::_mm_sll_epi32
    arch::x86::_mm_sll_epi64
    arch::x86::_mm_slli_epi16
    arch::x86::_mm_slli_epi32
    arch::x86::_mm_slli_epi64
    arch::x86::_mm_slli_si128
    arch::x86::_mm_sllv_epi32
    arch::x86::_mm_sllv_epi64
    arch::x86::_mm_sqrt_pd
    arch::x86::_mm_sqrt_ps
    arch::x86::_mm_sqrt_sd
    arch::x86::_mm_sqrt_ss
    arch::x86::_mm_sra_epi16
    arch::x86::_mm_sra_epi32
    arch::x86::_mm_srai_epi16
    arch::x86::_mm_srai_epi32
    arch::x86::_mm_srav_epi32
    arch::x86::_mm_srl_epi16
    arch::x86::_mm_srl_epi32
    arch::x86::_mm_srl_epi64
    arch::x86::_mm_srli_epi16
    arch::x86::_mm_srli_epi32
    arch::x86::_mm_srli_epi64
    arch::x86::_mm_srli_si128
    arch::x86::_mm_srlv_epi32
    arch::x86::_mm_srlv_epi64
    arch::x86::_mm_store1_pd
    arch::x86::_mm_store1_ps
    arch::x86::_mm_store_pd
    arch::x86::_mm_store_pd1
    arch::x86::_mm_store_ps
    arch::x86::_mm_store_ps1
    arch::x86::_mm_store_sd
    arch::x86::_mm_store_si128
    arch::x86::_mm_store_ss
    arch::x86::_mm_storeh_pd
    arch::x86::_mm_storeh_pi
    arch::x86::_mm_storel_epi64
    arch::x86::_mm_storel_pd
    arch::x86::_mm_storel_pi
    arch::x86::_mm_storer_pd
    arch::x86::_mm_storer_ps
    arch::x86::_mm_storeu_pd
    arch::x86::_mm_storeu_ps
    arch::x86::_mm_storeu_si128
    arch::x86::_mm_stream_pd
    arch::x86::_mm_stream_pi
    arch::x86::_mm_stream_ps
    arch::x86::_mm_stream_sd
    arch::x86::_mm_stream_si128
    arch::x86::_mm_stream_si32
    arch::x86::_mm_stream_ss
    arch::x86::_mm_sub_epi16
    arch::x86::_mm_sub_epi32
    arch::x86::_mm_sub_epi64
    arch::x86::_mm_sub_epi8
    arch::x86::_mm_sub_pd
    arch::x86::_mm_sub_pi16
    arch::x86::_mm_sub_pi32
    arch::x86::_mm_sub_pi8
    arch::x86::_mm_sub_ps
    arch::x86::_mm_sub_sd
    arch::x86::_mm_sub_si64
    arch::x86::_mm_sub_ss
    arch::x86::_mm_subs_epi16
    arch::x86::_mm_subs_epi8
    arch::x86::_mm_subs_epu16
    arch::x86::_mm_subs_epu8
    arch::x86::_mm_subs_pi16
    arch::x86::_mm_subs_pi8
    arch::x86::_mm_subs_pu16
    arch::x86::_mm_subs_pu8
    arch::x86::_mm_test_all_ones
    arch::x86::_mm_test_all_zeros
    arch::x86::_mm_test_mix_ones_zeros
    arch::x86::_mm_testc_pd
    arch::x86::_mm_testc_ps
    arch::x86::_mm_testc_si128
    arch::x86::_mm_testnzc_pd
    arch::x86::_mm_testnzc_ps
    arch::x86::_mm_testnzc_si128
    arch::x86::_mm_testz_pd
    arch::x86::_mm_testz_ps
    arch::x86::_mm_testz_si128
    arch::x86::_mm_tzcnt_32
    arch::x86::_mm_ucomieq_sd
    arch::x86::_mm_ucomieq_ss
    arch::x86::_mm_ucomige_sd
    arch::x86::_mm_ucomige_ss
    arch::x86::_mm_ucomigt_sd
    arch::x86::_mm_ucomigt_ss
    arch::x86::_mm_ucomile_sd
    arch::x86::_mm_ucomile_ss
    arch::x86::_mm_ucomilt_sd
    arch::x86::_mm_ucomilt_ss
    arch::x86::_mm_ucomineq_sd
    arch::x86::_mm_ucomineq_ss
    arch::x86::_mm_undefined_pd
    arch::x86::_mm_undefined_ps
    arch::x86::_mm_undefined_si128
    arch::x86::_mm_unpackhi_epi16
    arch::x86::_mm_unpackhi_epi32
    arch::x86::_mm_unpackhi_epi64
    arch::x86::_mm_unpackhi_epi8
    arch::x86::_mm_unpackhi_pd
    arch::x86::_mm_unpackhi_pi16
    arch::x86::_mm_unpackhi_pi32
    arch::x86::_mm_unpackhi_pi8
    arch::x86::_mm_unpackhi_ps
    arch::x86::_mm_unpacklo_epi16
    arch::x86::_mm_unpacklo_epi32
    arch::x86::_mm_unpacklo_epi64
    arch::x86::_mm_unpacklo_epi8
    arch::x86::_mm_unpacklo_pd
    arch::x86::_mm_unpacklo_pi16
    arch::x86::_mm_unpacklo_pi32
    arch::x86::_mm_unpacklo_pi8
    arch::x86::_mm_unpacklo_ps
    arch::x86::_mm_xor_pd
    arch::x86::_mm_xor_ps
    arch::x86::_mm_xor_si128
    arch::x86::_mulx_u32
    arch::x86::_pdep_u32
    arch::x86::_pext_u32
    arch::x86::_popcnt32
    arch::x86::_rdrand16_step
    arch::x86::_rdrand32_step
    arch::x86::_rdseed16_step
    arch::x86::_rdseed32_step
    arch::x86::_rdtsc
    arch::x86::_t1mskc_u32
    arch::x86::_t1mskc_u64
    arch::x86::_tzcnt_u32
    arch::x86::_tzmsk_u32
    arch::x86::_tzmsk_u64
    arch::x86::_xgetbv
    arch::x86::_xrstor
    arch::x86::_xrstors
    arch::x86::_xsave
    arch::x86::_xsavec
    arch::x86::_xsaveopt
    arch::x86::_xsaves
    arch::x86::_xsetbv
    arch::x86::has_cpuid
    arch::x86_64::_MM_GET_EXCEPTION_MASK
    arch::x86_64::_MM_GET_EXCEPTION_STATE
    arch::x86_64::_MM_GET_FLUSH_ZERO_MODE
    arch::x86_64::_MM_GET_ROUNDING_MODE
    arch::x86_64::_MM_SET_EXCEPTION_MASK
    arch::x86_64::_MM_SET_EXCEPTION_STATE
    arch::x86_64::_MM_SET_FLUSH_ZERO_MODE
    arch::x86_64::_MM_SET_ROUNDING_MODE
    arch::x86_64::_MM_TRANSPOSE4_PS
    arch::x86_64::__cpuid
    arch::x86_64::__cpuid_count
    arch::x86_64::__get_cpuid_max
    arch::x86_64::__rdtscp
    arch::x86_64::__readeflags
    arch::x86_64::__writeeflags
    arch::x86_64::_andn_u32
    arch::x86_64::_andn_u64
    arch::x86_64::_bextr2_u32
    arch::x86_64::_bextr2_u64
    arch::x86_64::_bextr_u32
    arch::x86_64::_bextr_u64
    arch::x86_64::_blcfill_u32
    arch::x86_64::_blcfill_u64
    arch::x86_64::_blci_u32
    arch::x86_64::_blci_u64
    arch::x86_64::_blcic_u32
    arch::x86_64::_blcic_u64
    arch::x86_64::_blcmsk_u32
    arch::x86_64::_blcmsk_u64
    arch::x86_64::_blcs_u32
    arch::x86_64::_blcs_u64
    arch::x86_64::_blsfill_u32
    arch::x86_64::_blsfill_u64
    arch::x86_64::_blsi_u32
    arch::x86_64::_blsi_u64
    arch::x86_64::_blsic_u32
    arch::x86_64::_blsic_u64
    arch::x86_64::_blsmsk_u32
    arch::x86_64::_blsmsk_u64
    arch::x86_64::_blsr_u32
    arch::x86_64::_blsr_u64
    arch::x86_64::_bswap
    arch::x86_64::_bswap64
    arch::x86_64::_bzhi_u32
    arch::x86_64::_bzhi_u64
    arch::x86_64::_fxrstor
    arch::x86_64::_fxrstor64
    arch::x86_64::_fxsave
    arch::x86_64::_fxsave64
    arch::x86_64::_lzcnt_u32
    arch::x86_64::_lzcnt_u64
    arch::x86_64::_m_maskmovq
    arch::x86_64::_m_paddb
    arch::x86_64::_m_paddd
    arch::x86_64::_m_paddsb
    arch::x86_64::_m_paddsw
    arch::x86_64::_m_paddusb
    arch::x86_64::_m_paddusw
    arch::x86_64::_m_paddw
    arch::x86_64::_m_pavgb
    arch::x86_64::_m_pavgw
    arch::x86_64::_m_pextrw
    arch::x86_64::_m_pinsrw
    arch::x86_64::_m_pmaxsw
    arch::x86_64::_m_pmaxub
    arch::x86_64::_m_pminsw
    arch::x86_64::_m_pminub
    arch::x86_64::_m_pmovmskb
    arch::x86_64::_m_pmulhuw
    arch::x86_64::_m_psadbw
    arch::x86_64::_m_pshufw
    arch::x86_64::_m_psubb
    arch::x86_64::_m_psubd
    arch::x86_64::_m_psubsb
    arch::x86_64::_m_psubsw
    arch::x86_64::_m_psubusb
    arch::x86_64::_m_psubusw
    arch::x86_64::_m_psubw
    arch::x86_64::_mm256_abs_epi16
    arch::x86_64::_mm256_abs_epi32
    arch::x86_64::_mm256_abs_epi8
    arch::x86_64::_mm256_add_epi16
    arch::x86_64::_mm256_add_epi32
    arch::x86_64::_mm256_add_epi64
    arch::x86_64::_mm256_add_epi8
    arch::x86_64::_mm256_add_pd
    arch::x86_64::_mm256_add_ps
    arch::x86_64::_mm256_adds_epi16
    arch::x86_64::_mm256_adds_epi8
    arch::x86_64::_mm256_adds_epu16
    arch::x86_64::_mm256_adds_epu8
    arch::x86_64::_mm256_addsub_pd
    arch::x86_64::_mm256_addsub_ps
    arch::x86_64::_mm256_alignr_epi8
    arch::x86_64::_mm256_and_pd
    arch::x86_64::_mm256_and_ps
    arch::x86_64::_mm256_and_si256
    arch::x86_64::_mm256_andnot_pd
    arch::x86_64::_mm256_andnot_ps
    arch::x86_64::_mm256_andnot_si256
    arch::x86_64::_mm256_avg_epu16
    arch::x86_64::_mm256_avg_epu8
    arch::x86_64::_mm256_blend_epi16
    arch::x86_64::_mm256_blend_epi32
    arch::x86_64::_mm256_blend_pd
    arch::x86_64::_mm256_blend_ps
    arch::x86_64::_mm256_blendv_epi8
    arch::x86_64::_mm256_blendv_pd
    arch::x86_64::_mm256_blendv_ps
    arch::x86_64::_mm256_broadcast_pd
    arch::x86_64::_mm256_broadcast_ps
    arch::x86_64::_mm256_broadcast_sd
    arch::x86_64::_mm256_broadcast_ss
    arch::x86_64::_mm256_broadcastb_epi8
    arch::x86_64::_mm256_broadcastd_epi32
    arch::x86_64::_mm256_broadcastq_epi64
    arch::x86_64::_mm256_broadcastsd_pd
    arch::x86_64::_mm256_broadcastsi128_si256
    arch::x86_64::_mm256_broadcastss_ps
    arch::x86_64::_mm256_broadcastw_epi16
    arch::x86_64::_mm256_bslli_epi128
    arch::x86_64::_mm256_bsrli_epi128
    arch::x86_64::_mm256_castpd128_pd256
    arch::x86_64::_mm256_castpd256_pd128
    arch::x86_64::_mm256_castpd_ps
    arch::x86_64::_mm256_castpd_si256
    arch::x86_64::_mm256_castps128_ps256
    arch::x86_64::_mm256_castps256_ps128
    arch::x86_64::_mm256_castps_pd
    arch::x86_64::_mm256_castps_si256
    arch::x86_64::_mm256_castsi128_si256
    arch::x86_64::_mm256_castsi256_pd
    arch::x86_64::_mm256_castsi256_ps
    arch::x86_64::_mm256_castsi256_si128
    arch::x86_64::_mm256_ceil_pd
    arch::x86_64::_mm256_ceil_ps
    arch::x86_64::_mm256_cmp_pd
    arch::x86_64::_mm256_cmp_ps
    arch::x86_64::_mm256_cmpeq_epi16
    arch::x86_64::_mm256_cmpeq_epi32
    arch::x86_64::_mm256_cmpeq_epi64
    arch::x86_64::_mm256_cmpeq_epi8
    arch::x86_64::_mm256_cmpgt_epi16
    arch::x86_64::_mm256_cmpgt_epi32
    arch::x86_64::_mm256_cmpgt_epi64
    arch::x86_64::_mm256_cmpgt_epi8
    arch::x86_64::_mm256_cvtepi16_epi32
    arch::x86_64::_mm256_cvtepi16_epi64
    arch::x86_64::_mm256_cvtepi32_epi64
    arch::x86_64::_mm256_cvtepi32_pd
    arch::x86_64::_mm256_cvtepi32_ps
    arch::x86_64::_mm256_cvtepi8_epi16
    arch::x86_64::_mm256_cvtepi8_epi32
    arch::x86_64::_mm256_cvtepi8_epi64
    arch::x86_64::_mm256_cvtepu16_epi32
    arch::x86_64::_mm256_cvtepu16_epi64
    arch::x86_64::_mm256_cvtepu32_epi64
    arch::x86_64::_mm256_cvtepu8_epi16
    arch::x86_64::_mm256_cvtepu8_epi32
    arch::x86_64::_mm256_cvtepu8_epi64
    arch::x86_64::_mm256_cvtpd_epi32
    arch::x86_64::_mm256_cvtpd_ps
    arch::x86_64::_mm256_cvtps_epi32
    arch::x86_64::_mm256_cvtps_pd
    arch::x86_64::_mm256_cvtsd_f64
    arch::x86_64::_mm256_cvtsi256_si32
    arch::x86_64::_mm256_cvtss_f32
    arch::x86_64::_mm256_cvttpd_epi32
    arch::x86_64::_mm256_cvttps_epi32
    arch::x86_64::_mm256_div_pd
    arch::x86_64::_mm256_div_ps
    arch::x86_64::_mm256_dp_ps
    arch::x86_64::_mm256_extract_epi16
    arch::x86_64::_mm256_extract_epi32
    arch::x86_64::_mm256_extract_epi64
    arch::x86_64::_mm256_extract_epi8
    arch::x86_64::_mm256_extractf128_pd
    arch::x86_64::_mm256_extractf128_ps
    arch::x86_64::_mm256_extractf128_si256
    arch::x86_64::_mm256_extracti128_si256
    arch::x86_64::_mm256_floor_pd
    arch::x86_64::_mm256_floor_ps
    arch::x86_64::_mm256_fmadd_pd
    arch::x86_64::_mm256_fmadd_ps
    arch::x86_64::_mm256_fmaddsub_pd
    arch::x86_64::_mm256_fmaddsub_ps
    arch::x86_64::_mm256_fmsub_pd
    arch::x86_64::_mm256_fmsub_ps
    arch::x86_64::_mm256_fmsubadd_pd
    arch::x86_64::_mm256_fmsubadd_ps
    arch::x86_64::_mm256_fnmadd_pd
    arch::x86_64::_mm256_fnmadd_ps
    arch::x86_64::_mm256_fnmsub_pd
    arch::x86_64::_mm256_fnmsub_ps
    arch::x86_64::_mm256_hadd_epi16
    arch::x86_64::_mm256_hadd_epi32
    arch::x86_64::_mm256_hadd_pd
    arch::x86_64::_mm256_hadd_ps
    arch::x86_64::_mm256_hadds_epi16
    arch::x86_64::_mm256_hsub_epi16
    arch::x86_64::_mm256_hsub_epi32
    arch::x86_64::_mm256_hsub_pd
    arch::x86_64::_mm256_hsub_ps
    arch::x86_64::_mm256_hsubs_epi16
    arch::x86_64::_mm256_i32gather_epi32
    arch::x86_64::_mm256_i32gather_epi64
    arch::x86_64::_mm256_i32gather_pd
    arch::x86_64::_mm256_i32gather_ps
    arch::x86_64::_mm256_i64gather_epi32
    arch::x86_64::_mm256_i64gather_epi64
    arch::x86_64::_mm256_i64gather_pd
    arch::x86_64::_mm256_i64gather_ps
    arch::x86_64::_mm256_insert_epi16
    arch::x86_64::_mm256_insert_epi32
    arch::x86_64::_mm256_insert_epi64
    arch::x86_64::_mm256_insert_epi8
    arch::x86_64::_mm256_insertf128_pd
    arch::x86_64::_mm256_insertf128_ps
    arch::x86_64::_mm256_insertf128_si256
    arch::x86_64::_mm256_inserti128_si256
    arch::x86_64::_mm256_lddqu_si256
    arch::x86_64::_mm256_load_pd
    arch::x86_64::_mm256_load_ps
    arch::x86_64::_mm256_load_si256
    arch::x86_64::_mm256_loadu2_m128
    arch::x86_64::_mm256_loadu2_m128d
    arch::x86_64::_mm256_loadu2_m128i
    arch::x86_64::_mm256_loadu_pd
    arch::x86_64::_mm256_loadu_ps
    arch::x86_64::_mm256_loadu_si256
    arch::x86_64::_mm256_madd_epi16
    arch::x86_64::_mm256_maddubs_epi16
    arch::x86_64::_mm256_mask_i32gather_epi32
    arch::x86_64::_mm256_mask_i32gather_epi64
    arch::x86_64::_mm256_mask_i32gather_pd
    arch::x86_64::_mm256_mask_i32gather_ps
    arch::x86_64::_mm256_mask_i64gather_epi32
    arch::x86_64::_mm256_mask_i64gather_epi64
    arch::x86_64::_mm256_mask_i64gather_pd
    arch::x86_64::_mm256_mask_i64gather_ps
    arch::x86_64::_mm256_maskload_epi32
    arch::x86_64::_mm256_maskload_epi64
    arch::x86_64::_mm256_maskload_pd
    arch::x86_64::_mm256_maskload_ps
    arch::x86_64::_mm256_maskstore_epi32
    arch::x86_64::_mm256_maskstore_epi64
    arch::x86_64::_mm256_maskstore_pd
    arch::x86_64::_mm256_maskstore_ps
    arch::x86_64::_mm256_max_epi16
    arch::x86_64::_mm256_max_epi32
    arch::x86_64::_mm256_max_epi8
    arch::x86_64::_mm256_max_epu16
    arch::x86_64::_mm256_max_epu32
    arch::x86_64::_mm256_max_epu8
    arch::x86_64::_mm256_max_pd
    arch::x86_64::_mm256_max_ps
    arch::x86_64::_mm256_min_epi16
    arch::x86_64::_mm256_min_epi32
    arch::x86_64::_mm256_min_epi8
    arch::x86_64::_mm256_min_epu16
    arch::x86_64::_mm256_min_epu32
    arch::x86_64::_mm256_min_epu8
    arch::x86_64::_mm256_min_pd
    arch::x86_64::_mm256_min_ps
    arch::x86_64::_mm256_movedup_pd
    arch::x86_64::_mm256_movehdup_ps
    arch::x86_64::_mm256_moveldup_ps
    arch::x86_64::_mm256_movemask_epi8
    arch::x86_64::_mm256_movemask_pd
    arch::x86_64::_mm256_movemask_ps
    arch::x86_64::_mm256_mpsadbw_epu8
    arch::x86_64::_mm256_mul_epi32
    arch::x86_64::_mm256_mul_epu32
    arch::x86_64::_mm256_mul_pd
    arch::x86_64::_mm256_mul_ps
    arch::x86_64::_mm256_mulhi_epi16
    arch::x86_64::_mm256_mulhi_epu16
    arch::x86_64::_mm256_mulhrs_epi16
    arch::x86_64::_mm256_mullo_epi16
    arch::x86_64::_mm256_mullo_epi32
    arch::x86_64::_mm256_or_pd
    arch::x86_64::_mm256_or_ps
    arch::x86_64::_mm256_or_si256
    arch::x86_64::_mm256_packs_epi16
    arch::x86_64::_mm256_packs_epi32
    arch::x86_64::_mm256_packus_epi16
    arch::x86_64::_mm256_packus_epi32
    arch::x86_64::_mm256_permute2f128_pd
    arch::x86_64::_mm256_permute2f128_ps
    arch::x86_64::_mm256_permute2f128_si256
    arch::x86_64::_mm256_permute2x128_si256
    arch::x86_64::_mm256_permute4x64_epi64
    arch::x86_64::_mm256_permute4x64_pd
    arch::x86_64::_mm256_permute_pd
    arch::x86_64::_mm256_permute_ps
    arch::x86_64::_mm256_permutevar8x32_epi32
    arch::x86_64::_mm256_permutevar8x32_ps
    arch::x86_64::_mm256_permutevar_pd
    arch::x86_64::_mm256_permutevar_ps
    arch::x86_64::_mm256_rcp_ps
    arch::x86_64::_mm256_round_pd
    arch::x86_64::_mm256_round_ps
    arch::x86_64::_mm256_rsqrt_ps
    arch::x86_64::_mm256_sad_epu8
    arch::x86_64::_mm256_set1_epi16
    arch::x86_64::_mm256_set1_epi32
    arch::x86_64::_mm256_set1_epi64x
    arch::x86_64::_mm256_set1_epi8
    arch::x86_64::_mm256_set1_pd
    arch::x86_64::_mm256_set1_ps
    arch::x86_64::_mm256_set_epi16
    arch::x86_64::_mm256_set_epi32
    arch::x86_64::_mm256_set_epi64x
    arch::x86_64::_mm256_set_epi8
    arch::x86_64::_mm256_set_m128
    arch::x86_64::_mm256_set_m128d
    arch::x86_64::_mm256_set_m128i
    arch::x86_64::_mm256_set_pd
    arch::x86_64::_mm256_set_ps
    arch::x86_64::_mm256_setr_epi16
    arch::x86_64::_mm256_setr_epi32
    arch::x86_64::_mm256_setr_epi64x
    arch::x86_64::_mm256_setr_epi8
    arch::x86_64::_mm256_setr_m128
    arch::x86_64::_mm256_setr_m128d
    arch::x86_64::_mm256_setr_m128i
    arch::x86_64::_mm256_setr_pd
    arch::x86_64::_mm256_setr_ps
    arch::x86_64::_mm256_setzero_pd
    arch::x86_64::_mm256_setzero_ps
    arch::x86_64::_mm256_setzero_si256
    arch::x86_64::_mm256_shuffle_epi32
    arch::x86_64::_mm256_shuffle_epi8
    arch::x86_64::_mm256_shuffle_pd
    arch::x86_64::_mm256_shuffle_ps
    arch::x86_64::_mm256_shufflehi_epi16
    arch::x86_64::_mm256_shufflelo_epi16
    arch::x86_64::_mm256_sign_epi16
    arch::x86_64::_mm256_sign_epi32
    arch::x86_64::_mm256_sign_epi8
    arch::x86_64::_mm256_sll_epi16
    arch::x86_64::_mm256_sll_epi32
    arch::x86_64::_mm256_sll_epi64
    arch::x86_64::_mm256_slli_epi16
    arch::x86_64::_mm256_slli_epi32
    arch::x86_64::_mm256_slli_epi64
    arch::x86_64::_mm256_slli_si256
    arch::x86_64::_mm256_sllv_epi32
    arch::x86_64::_mm256_sllv_epi64
    arch::x86_64::_mm256_sqrt_pd
    arch::x86_64::_mm256_sqrt_ps
    arch::x86_64::_mm256_sra_epi16
    arch::x86_64::_mm256_sra_epi32
    arch::x86_64::_mm256_srai_epi16
    arch::x86_64::_mm256_srai_epi32
    arch::x86_64::_mm256_srav_epi32
    arch::x86_64::_mm256_srl_epi16
    arch::x86_64::_mm256_srl_epi32
    arch::x86_64::_mm256_srl_epi64
    arch::x86_64::_mm256_srli_epi16
    arch::x86_64::_mm256_srli_epi32
    arch::x86_64::_mm256_srli_epi64
    arch::x86_64::_mm256_srli_si256
    arch::x86_64::_mm256_srlv_epi32
    arch::x86_64::_mm256_srlv_epi64
    arch::x86_64::_mm256_store_pd
    arch::x86_64::_mm256_store_ps
    arch::x86_64::_mm256_store_si256
    arch::x86_64::_mm256_storeu2_m128
    arch::x86_64::_mm256_storeu2_m128d
    arch::x86_64::_mm256_storeu2_m128i
    arch::x86_64::_mm256_storeu_pd
    arch::x86_64::_mm256_storeu_ps
    arch::x86_64::_mm256_storeu_si256
    arch::x86_64::_mm256_stream_pd
    arch::x86_64::_mm256_stream_ps
    arch::x86_64::_mm256_stream_si256
    arch::x86_64::_mm256_sub_epi16
    arch::x86_64::_mm256_sub_epi32
    arch::x86_64::_mm256_sub_epi64
    arch::x86_64::_mm256_sub_epi8
    arch::x86_64::_mm256_sub_pd
    arch::x86_64::_mm256_sub_ps
    arch::x86_64::_mm256_subs_epi16
    arch::x86_64::_mm256_subs_epi8
    arch::x86_64::_mm256_subs_epu16
    arch::x86_64::_mm256_subs_epu8
    arch::x86_64::_mm256_testc_pd
    arch::x86_64::_mm256_testc_ps
    arch::x86_64::_mm256_testc_si256
    arch::x86_64::_mm256_testnzc_pd
    arch::x86_64::_mm256_testnzc_ps
    arch::x86_64::_mm256_testnzc_si256
    arch::x86_64::_mm256_testz_pd
    arch::x86_64::_mm256_testz_ps
    arch::x86_64::_mm256_testz_si256
    arch::x86_64::_mm256_undefined_pd
    arch::x86_64::_mm256_undefined_ps
    arch::x86_64::_mm256_undefined_si256
    arch::x86_64::_mm256_unpackhi_epi16
    arch::x86_64::_mm256_unpackhi_epi32
    arch::x86_64::_mm256_unpackhi_epi64
    arch::x86_64::_mm256_unpackhi_epi8
    arch::x86_64::_mm256_unpackhi_pd
    arch::x86_64::_mm256_unpackhi_ps
    arch::x86_64::_mm256_unpacklo_epi16
    arch::x86_64::_mm256_unpacklo_epi32
    arch::x86_64::_mm256_unpacklo_epi64
    arch::x86_64::_mm256_unpacklo_epi8
    arch::x86_64::_mm256_unpacklo_pd
    arch::x86_64::_mm256_unpacklo_ps
    arch::x86_64::_mm256_xor_pd
    arch::x86_64::_mm256_xor_ps
    arch::x86_64::_mm256_xor_si256
    arch::x86_64::_mm256_zeroall
    arch::x86_64::_mm256_zeroupper
    arch::x86_64::_mm256_zextpd128_pd256
    arch::x86_64::_mm256_zextps128_ps256
    arch::x86_64::_mm256_zextsi128_si256
    arch::x86_64::_mm_abs_epi16
    arch::x86_64::_mm_abs_epi32
    arch::x86_64::_mm_abs_epi8
    arch::x86_64::_mm_abs_pi16
    arch::x86_64::_mm_abs_pi32
    arch::x86_64::_mm_abs_pi8
    arch::x86_64::_mm_add_epi16
    arch::x86_64::_mm_add_epi32
    arch::x86_64::_mm_add_epi64
    arch::x86_64::_mm_add_epi8
    arch::x86_64::_mm_add_pd
    arch::x86_64::_mm_add_pi16
    arch::x86_64::_mm_add_pi32
    arch::x86_64::_mm_add_pi8
    arch::x86_64::_mm_add_ps
    arch::x86_64::_mm_add_sd
    arch::x86_64::_mm_add_si64
    arch::x86_64::_mm_add_ss
    arch::x86_64::_mm_adds_epi16
    arch::x86_64::_mm_adds_epi8
    arch::x86_64::_mm_adds_epu16
    arch::x86_64::_mm_adds_epu8
    arch::x86_64::_mm_adds_pi16
    arch::x86_64::_mm_adds_pi8
    arch::x86_64::_mm_adds_pu16
    arch::x86_64::_mm_adds_pu8
    arch::x86_64::_mm_addsub_pd
    arch::x86_64::_mm_addsub_ps
    arch::x86_64::_mm_aesdec_si128
    arch::x86_64::_mm_aesdeclast_si128
    arch::x86_64::_mm_aesenc_si128
    arch::x86_64::_mm_aesenclast_si128
    arch::x86_64::_mm_aesimc_si128
    arch::x86_64::_mm_aeskeygenassist_si128
    arch::x86_64::_mm_alignr_epi8
    arch::x86_64::_mm_alignr_pi8
    arch::x86_64::_mm_and_pd
    arch::x86_64::_mm_and_ps
    arch::x86_64::_mm_and_si128
    arch::x86_64::_mm_andnot_pd
    arch::x86_64::_mm_andnot_ps
    arch::x86_64::_mm_andnot_si128
    arch::x86_64::_mm_avg_epu16
    arch::x86_64::_mm_avg_epu8
    arch::x86_64::_mm_avg_pu16
    arch::x86_64::_mm_avg_pu8
    arch::x86_64::_mm_blend_epi16
    arch::x86_64::_mm_blend_epi32
    arch::x86_64::_mm_blend_pd
    arch::x86_64::_mm_blend_ps
    arch::x86_64::_mm_blendv_epi8
    arch::x86_64::_mm_blendv_pd
    arch::x86_64::_mm_blendv_ps
    arch::x86_64::_mm_broadcast_ss
    arch::x86_64::_mm_broadcastb_epi8
    arch::x86_64::_mm_broadcastd_epi32
    arch::x86_64::_mm_broadcastq_epi64
    arch::x86_64::_mm_broadcastsd_pd
    arch::x86_64::_mm_broadcastss_ps
    arch::x86_64::_mm_broadcastw_epi16
    arch::x86_64::_mm_bslli_si128
    arch::x86_64::_mm_bsrli_si128
    arch::x86_64::_mm_castpd_ps
    arch::x86_64::_mm_castpd_si128
    arch::x86_64::_mm_castps_pd
    arch::x86_64::_mm_castps_si128
    arch::x86_64::_mm_castsi128_pd
    arch::x86_64::_mm_castsi128_ps
    arch::x86_64::_mm_ceil_pd
    arch::x86_64::_mm_ceil_ps
    arch::x86_64::_mm_ceil_sd
    arch::x86_64::_mm_ceil_ss
    arch::x86_64::_mm_clflush
    arch::x86_64::_mm_clmulepi64_si128
    arch::x86_64::_mm_cmp_pd
    arch::x86_64::_mm_cmp_ps
    arch::x86_64::_mm_cmp_sd
    arch::x86_64::_mm_cmp_ss
    arch::x86_64::_mm_cmpeq_epi16
    arch::x86_64::_mm_cmpeq_epi32
    arch::x86_64::_mm_cmpeq_epi64
    arch::x86_64::_mm_cmpeq_epi8
    arch::x86_64::_mm_cmpeq_pd
    arch::x86_64::_mm_cmpeq_ps
    arch::x86_64::_mm_cmpeq_sd
    arch::x86_64::_mm_cmpeq_ss
    arch::x86_64::_mm_cmpestra
    arch::x86_64::_mm_cmpestrc
    arch::x86_64::_mm_cmpestri
    arch::x86_64::_mm_cmpestrm
    arch::x86_64::_mm_cmpestro
    arch::x86_64::_mm_cmpestrs
    arch::x86_64::_mm_cmpestrz
    arch::x86_64::_mm_cmpge_pd
    arch::x86_64::_mm_cmpge_ps
    arch::x86_64::_mm_cmpge_sd
    arch::x86_64::_mm_cmpge_ss
    arch::x86_64::_mm_cmpgt_epi16
    arch::x86_64::_mm_cmpgt_epi32
    arch::x86_64::_mm_cmpgt_epi64
    arch::x86_64::_mm_cmpgt_epi8
    arch::x86_64::_mm_cmpgt_pd
    arch::x86_64::_mm_cmpgt_pi16
    arch::x86_64::_mm_cmpgt_pi32
    arch::x86_64::_mm_cmpgt_pi8
    arch::x86_64::_mm_cmpgt_ps
    arch::x86_64::_mm_cmpgt_sd
    arch::x86_64::_mm_cmpgt_ss
    arch::x86_64::_mm_cmpistra
    arch::x86_64::_mm_cmpistrc
    arch::x86_64::_mm_cmpistri
    arch::x86_64::_mm_cmpistrm
    arch::x86_64::_mm_cmpistro
    arch::x86_64::_mm_cmpistrs
    arch::x86_64::_mm_cmpistrz
    arch::x86_64::_mm_cmple_pd
    arch::x86_64::_mm_cmple_ps
    arch::x86_64::_mm_cmple_sd
    arch::x86_64::_mm_cmple_ss
    arch::x86_64::_mm_cmplt_epi16
    arch::x86_64::_mm_cmplt_epi32
    arch::x86_64::_mm_cmplt_epi8
    arch::x86_64::_mm_cmplt_pd
    arch::x86_64::_mm_cmplt_ps
    arch::x86_64::_mm_cmplt_sd
    arch::x86_64::_mm_cmplt_ss
    arch::x86_64::_mm_cmpneq_pd
    arch::x86_64::_mm_cmpneq_ps
    arch::x86_64::_mm_cmpneq_sd
    arch::x86_64::_mm_cmpneq_ss
    arch::x86_64::_mm_cmpnge_pd
    arch::x86_64::_mm_cmpnge_ps
    arch::x86_64::_mm_cmpnge_sd
    arch::x86_64::_mm_cmpnge_ss
    arch::x86_64::_mm_cmpngt_pd
    arch::x86_64::_mm_cmpngt_ps
    arch::x86_64::_mm_cmpngt_sd
    arch::x86_64::_mm_cmpngt_ss
    arch::x86_64::_mm_cmpnle_pd
    arch::x86_64::_mm_cmpnle_ps
    arch::x86_64::_mm_cmpnle_sd
    arch::x86_64::_mm_cmpnle_ss
    arch::x86_64::_mm_cmpnlt_pd
    arch::x86_64::_mm_cmpnlt_ps
    arch::x86_64::_mm_cmpnlt_sd
    arch::x86_64::_mm_cmpnlt_ss
    arch::x86_64::_mm_cmpord_pd
    arch::x86_64::_mm_cmpord_ps
    arch::x86_64::_mm_cmpord_sd
    arch::x86_64::_mm_cmpord_ss
    arch::x86_64::_mm_cmpunord_pd
    arch::x86_64::_mm_cmpunord_ps
    arch::x86_64::_mm_cmpunord_sd
    arch::x86_64::_mm_cmpunord_ss
    arch::x86_64::_mm_comieq_sd
    arch::x86_64::_mm_comieq_ss
    arch::x86_64::_mm_comige_sd
    arch::x86_64::_mm_comige_ss
    arch::x86_64::_mm_comigt_sd
    arch::x86_64::_mm_comigt_ss
    arch::x86_64::_mm_comile_sd
    arch::x86_64::_mm_comile_ss
    arch::x86_64::_mm_comilt_sd
    arch::x86_64::_mm_comilt_ss
    arch::x86_64::_mm_comineq_sd
    arch::x86_64::_mm_comineq_ss
    arch::x86_64::_mm_crc32_u16
    arch::x86_64::_mm_crc32_u32
    arch::x86_64::_mm_crc32_u64
    arch::x86_64::_mm_crc32_u8
    arch::x86_64::_mm_cvt_pi2ps
    arch::x86_64::_mm_cvt_ps2pi
    arch::x86_64::_mm_cvt_si2ss
    arch::x86_64::_mm_cvt_ss2si
    arch::x86_64::_mm_cvtepi16_epi32
    arch::x86_64::_mm_cvtepi16_epi64
    arch::x86_64::_mm_cvtepi32_epi64
    arch::x86_64::_mm_cvtepi32_pd
    arch::x86_64::_mm_cvtepi32_ps
    arch::x86_64::_mm_cvtepi8_epi16
    arch::x86_64::_mm_cvtepi8_epi32
    arch::x86_64::_mm_cvtepi8_epi64
    arch::x86_64::_mm_cvtepu16_epi32
    arch::x86_64::_mm_cvtepu16_epi64
    arch::x86_64::_mm_cvtepu32_epi64
    arch::x86_64::_mm_cvtepu8_epi16
    arch::x86_64::_mm_cvtepu8_epi32
    arch::x86_64::_mm_cvtepu8_epi64
    arch::x86_64::_mm_cvtpd_epi32
    arch::x86_64::_mm_cvtpd_pi32
    arch::x86_64::_mm_cvtpd_ps
    arch::x86_64::_mm_cvtpi16_ps
    arch::x86_64::_mm_cvtpi32_pd
    arch::x86_64::_mm_cvtpi32_ps
    arch::x86_64::_mm_cvtpi32x2_ps
    arch::x86_64::_mm_cvtpi8_ps
    arch::x86_64::_mm_cvtps_epi32
    arch::x86_64::_mm_cvtps_pd
    arch::x86_64::_mm_cvtps_pi16
    arch::x86_64::_mm_cvtps_pi32
    arch::x86_64::_mm_cvtps_pi8
    arch::x86_64::_mm_cvtpu16_ps
    arch::x86_64::_mm_cvtpu8_ps
    arch::x86_64::_mm_cvtsd_f64
    arch::x86_64::_mm_cvtsd_si32
    arch::x86_64::_mm_cvtsd_si64
    arch::x86_64::_mm_cvtsd_si64x
    arch::x86_64::_mm_cvtsd_ss
    arch::x86_64::_mm_cvtsi128_si32
    arch::x86_64::_mm_cvtsi128_si64
    arch::x86_64::_mm_cvtsi128_si64x
    arch::x86_64::_mm_cvtsi32_sd
    arch::x86_64::_mm_cvtsi32_si128
    arch::x86_64::_mm_cvtsi32_ss
    arch::x86_64::_mm_cvtsi64_sd
    arch::x86_64::_mm_cvtsi64_si128
    arch::x86_64::_mm_cvtsi64_ss
    arch::x86_64::_mm_cvtsi64x_sd
    arch::x86_64::_mm_cvtsi64x_si128
    arch::x86_64::_mm_cvtss_f32
    arch::x86_64::_mm_cvtss_sd
    arch::x86_64::_mm_cvtss_si32
    arch::x86_64::_mm_cvtss_si64
    arch::x86_64::_mm_cvtt_ps2pi
    arch::x86_64::_mm_cvtt_ss2si
    arch::x86_64::_mm_cvttpd_epi32
    arch::x86_64::_mm_cvttpd_pi32
    arch::x86_64::_mm_cvttps_epi32
    arch::x86_64::_mm_cvttps_pi32
    arch::x86_64::_mm_cvttsd_si32
    arch::x86_64::_mm_cvttsd_si64
    arch::x86_64::_mm_cvttsd_si64x
    arch::x86_64::_mm_cvttss_si32
    arch::x86_64::_mm_cvttss_si64
    arch::x86_64::_mm_div_pd
    arch::x86_64::_mm_div_ps
    arch::x86_64::_mm_div_sd
    arch::x86_64::_mm_div_ss
    arch::x86_64::_mm_dp_pd
    arch::x86_64::_mm_dp_ps
    arch::x86_64::_mm_extract_epi16
    arch::x86_64::_mm_extract_epi32
    arch::x86_64::_mm_extract_epi64
    arch::x86_64::_mm_extract_epi8
    arch::x86_64::_mm_extract_pi16
    arch::x86_64::_mm_extract_ps
    arch::x86_64::_mm_extract_si64
    arch::x86_64::_mm_floor_pd
    arch::x86_64::_mm_floor_ps
    arch::x86_64::_mm_floor_sd
    arch::x86_64::_mm_floor_ss
    arch::x86_64::_mm_fmadd_pd
    arch::x86_64::_mm_fmadd_ps
    arch::x86_64::_mm_fmadd_sd
    arch::x86_64::_mm_fmadd_ss
    arch::x86_64::_mm_fmaddsub_pd
    arch::x86_64::_mm_fmaddsub_ps
    arch::x86_64::_mm_fmsub_pd
    arch::x86_64::_mm_fmsub_ps
    arch::x86_64::_mm_fmsub_sd
    arch::x86_64::_mm_fmsub_ss
    arch::x86_64::_mm_fmsubadd_pd
    arch::x86_64::_mm_fmsubadd_ps
    arch::x86_64::_mm_fnmadd_pd
    arch::x86_64::_mm_fnmadd_ps
    arch::x86_64::_mm_fnmadd_sd
    arch::x86_64::_mm_fnmadd_ss
    arch::x86_64::_mm_fnmsub_pd
    arch::x86_64::_mm_fnmsub_ps
    arch::x86_64::_mm_fnmsub_sd
    arch::x86_64::_mm_fnmsub_ss
    arch::x86_64::_mm_getcsr
    arch::x86_64::_mm_hadd_epi16
    arch::x86_64::_mm_hadd_epi32
    arch::x86_64::_mm_hadd_pd
    arch::x86_64::_mm_hadd_pi16
    arch::x86_64::_mm_hadd_pi32
    arch::x86_64::_mm_hadd_ps
    arch::x86_64::_mm_hadds_epi16
    arch::x86_64::_mm_hadds_pi16
    arch::x86_64::_mm_hsub_epi16
    arch::x86_64::_mm_hsub_epi32
    arch::x86_64::_mm_hsub_pd
    arch::x86_64::_mm_hsub_pi16
    arch::x86_64::_mm_hsub_pi32
    arch::x86_64::_mm_hsub_ps
    arch::x86_64::_mm_hsubs_epi16
    arch::x86_64::_mm_hsubs_pi16
    arch::x86_64::_mm_i32gather_epi32
    arch::x86_64::_mm_i32gather_epi64
    arch::x86_64::_mm_i32gather_pd
    arch::x86_64::_mm_i32gather_ps
    arch::x86_64::_mm_i64gather_epi32
    arch::x86_64::_mm_i64gather_epi64
    arch::x86_64::_mm_i64gather_pd
    arch::x86_64::_mm_i64gather_ps
    arch::x86_64::_mm_insert_epi16
    arch::x86_64::_mm_insert_epi32
    arch::x86_64::_mm_insert_epi64
    arch::x86_64::_mm_insert_epi8
    arch::x86_64::_mm_insert_pi16
    arch::x86_64::_mm_insert_ps
    arch::x86_64::_mm_insert_si64
    arch::x86_64::_mm_lddqu_si128
    arch::x86_64::_mm_lfence
    arch::x86_64::_mm_load1_pd
    arch::x86_64::_mm_load1_ps
    arch::x86_64::_mm_load_pd
    arch::x86_64::_mm_load_pd1
    arch::x86_64::_mm_load_ps
    arch::x86_64::_mm_load_ps1
    arch::x86_64::_mm_load_sd
    arch::x86_64::_mm_load_si128
    arch::x86_64::_mm_load_ss
    arch::x86_64::_mm_loaddup_pd
    arch::x86_64::_mm_loadh_pd
    arch::x86_64::_mm_loadh_pi
    arch::x86_64::_mm_loadl_epi64
    arch::x86_64::_mm_loadl_pd
    arch::x86_64::_mm_loadl_pi
    arch::x86_64::_mm_loadr_pd
    arch::x86_64::_mm_loadr_ps
    arch::x86_64::_mm_loadu_pd
    arch::x86_64::_mm_loadu_ps
    arch::x86_64::_mm_loadu_si128
    arch::x86_64::_mm_madd_epi16
    arch::x86_64::_mm_maddubs_epi16
    arch::x86_64::_mm_maddubs_pi16
    arch::x86_64::_mm_mask_i32gather_epi32
    arch::x86_64::_mm_mask_i32gather_epi64
    arch::x86_64::_mm_mask_i32gather_pd
    arch::x86_64::_mm_mask_i32gather_ps
    arch::x86_64::_mm_mask_i64gather_epi32
    arch::x86_64::_mm_mask_i64gather_epi64
    arch::x86_64::_mm_mask_i64gather_pd
    arch::x86_64::_mm_mask_i64gather_ps
    arch::x86_64::_mm_maskload_epi32
    arch::x86_64::_mm_maskload_epi64
    arch::x86_64::_mm_maskload_pd
    arch::x86_64::_mm_maskload_ps
    arch::x86_64::_mm_maskmove_si64
    arch::x86_64::_mm_maskmoveu_si128
    arch::x86_64::_mm_maskstore_epi32
    arch::x86_64::_mm_maskstore_epi64
    arch::x86_64::_mm_maskstore_pd
    arch::x86_64::_mm_maskstore_ps
    arch::x86_64::_mm_max_epi16
    arch::x86_64::_mm_max_epi32
    arch::x86_64::_mm_max_epi8
    arch::x86_64::_mm_max_epu16
    arch::x86_64::_mm_max_epu32
    arch::x86_64::_mm_max_epu8
    arch::x86_64::_mm_max_pd
    arch::x86_64::_mm_max_pi16
    arch::x86_64::_mm_max_ps
    arch::x86_64::_mm_max_pu8
    arch::x86_64::_mm_max_sd
    arch::x86_64::_mm_max_ss
    arch::x86_64::_mm_mfence
    arch::x86_64::_mm_min_epi16
    arch::x86_64::_mm_min_epi32
    arch::x86_64::_mm_min_epi8
    arch::x86_64::_mm_min_epu16
    arch::x86_64::_mm_min_epu32
    arch::x86_64::_mm_min_epu8
    arch::x86_64::_mm_min_pd
    arch::x86_64::_mm_min_pi16
    arch::x86_64::_mm_min_ps
    arch::x86_64::_mm_min_pu8
    arch::x86_64::_mm_min_sd
    arch::x86_64::_mm_min_ss
    arch::x86_64::_mm_minpos_epu16
    arch::x86_64::_mm_move_epi64
    arch::x86_64::_mm_move_sd
    arch::x86_64::_mm_move_ss
    arch::x86_64::_mm_movedup_pd
    arch::x86_64::_mm_movehdup_ps
    arch::x86_64::_mm_movehl_ps
    arch::x86_64::_mm_moveldup_ps
    arch::x86_64::_mm_movelh_ps
    arch::x86_64::_mm_movemask_epi8
    arch::x86_64::_mm_movemask_pd
    arch::x86_64::_mm_movemask_pi8
    arch::x86_64::_mm_movemask_ps
    arch::x86_64::_mm_movepi64_pi64
    arch::x86_64::_mm_movpi64_epi64
    arch::x86_64::_mm_mpsadbw_epu8
    arch::x86_64::_mm_mul_epi32
    arch::x86_64::_mm_mul_epu32
    arch::x86_64::_mm_mul_pd
    arch::x86_64::_mm_mul_ps
    arch::x86_64::_mm_mul_sd
    arch::x86_64::_mm_mul_ss
    arch::x86_64::_mm_mul_su32
    arch::x86_64::_mm_mulhi_epi16
    arch::x86_64::_mm_mulhi_epu16
    arch::x86_64::_mm_mulhi_pu16
    arch::x86_64::_mm_mulhrs_epi16
    arch::x86_64::_mm_mulhrs_pi16
    arch::x86_64::_mm_mullo_epi16
    arch::x86_64::_mm_mullo_epi32
    arch::x86_64::_mm_or_pd
    arch::x86_64::_mm_or_ps
    arch::x86_64::_mm_or_si128
    arch::x86_64::_mm_packs_epi16
    arch::x86_64::_mm_packs_epi32
    arch::x86_64::_mm_packs_pi16
    arch::x86_64::_mm_packs_pi32
    arch::x86_64::_mm_packus_epi16
    arch::x86_64::_mm_packus_epi32
    arch::x86_64::_mm_pause
    arch::x86_64::_mm_permute_pd
    arch::x86_64::_mm_permute_ps
    arch::x86_64::_mm_permutevar_pd
    arch::x86_64::_mm_permutevar_ps
    arch::x86_64::_mm_prefetch
    arch::x86_64::_mm_rcp_ps
    arch::x86_64::_mm_rcp_ss
    arch::x86_64::_mm_round_pd
    arch::x86_64::_mm_round_ps
    arch::x86_64::_mm_round_sd
    arch::x86_64::_mm_round_ss
    arch::x86_64::_mm_rsqrt_ps
    arch::x86_64::_mm_rsqrt_ss
    arch::x86_64::_mm_sad_epu8
    arch::x86_64::_mm_sad_pu8
    arch::x86_64::_mm_set1_epi16
    arch::x86_64::_mm_set1_epi32
    arch::x86_64::_mm_set1_epi64
    arch::x86_64::_mm_set1_epi64x
    arch::x86_64::_mm_set1_epi8
    arch::x86_64::_mm_set1_pd
    arch::x86_64::_mm_set1_pi16
    arch::x86_64::_mm_set1_pi32
    arch::x86_64::_mm_set1_pi8
    arch::x86_64::_mm_set1_ps
    arch::x86_64::_mm_set_epi16
    arch::x86_64::_mm_set_epi32
    arch::x86_64::_mm_set_epi64
    arch::x86_64::_mm_set_epi64x
    arch::x86_64::_mm_set_epi8
    arch::x86_64::_mm_set_pd
    arch::x86_64::_mm_set_pd1
    arch::x86_64::_mm_set_pi16
    arch::x86_64::_mm_set_pi32
    arch::x86_64::_mm_set_pi8
    arch::x86_64::_mm_set_ps
    arch::x86_64::_mm_set_ps1
    arch::x86_64::_mm_set_sd
    arch::x86_64::_mm_set_ss
    arch::x86_64::_mm_setcsr
    arch::x86_64::_mm_setr_epi16
    arch::x86_64::_mm_setr_epi32
    arch::x86_64::_mm_setr_epi64
    arch::x86_64::_mm_setr_epi8
    arch::x86_64::_mm_setr_pd
    arch::x86_64::_mm_setr_pi16
    arch::x86_64::_mm_setr_pi32
    arch::x86_64::_mm_setr_pi8
    arch::x86_64::_mm_setr_ps
    arch::x86_64::_mm_setzero_pd
    arch::x86_64::_mm_setzero_ps
    arch::x86_64::_mm_setzero_si128
    arch::x86_64::_mm_setzero_si64
    arch::x86_64::_mm_sfence
    arch::x86_64::_mm_sha1msg1_epu32
    arch::x86_64::_mm_sha1msg2_epu32
    arch::x86_64::_mm_sha1nexte_epu32
    arch::x86_64::_mm_sha1rnds4_epu32
    arch::x86_64::_mm_sha256msg1_epu32
    arch::x86_64::_mm_sha256msg2_epu32
    arch::x86_64::_mm_sha256rnds2_epu32
    arch::x86_64::_mm_shuffle_epi32
    arch::x86_64::_mm_shuffle_epi8
    arch::x86_64::_mm_shuffle_pd
    arch::x86_64::_mm_shuffle_pi16
    arch::x86_64::_mm_shuffle_pi8
    arch::x86_64::_mm_shuffle_ps
    arch::x86_64::_mm_shufflehi_epi16
    arch::x86_64::_mm_shufflelo_epi16
    arch::x86_64::_mm_sign_epi16
    arch::x86_64::_mm_sign_epi32
    arch::x86_64::_mm_sign_epi8
    arch::x86_64::_mm_sign_pi16
    arch::x86_64::_mm_sign_pi32
    arch::x86_64::_mm_sign_pi8
    arch::x86_64::_mm_sll_epi16
    arch::x86_64::_mm_sll_epi32
    arch::x86_64::_mm_sll_epi64
    arch::x86_64::_mm_slli_epi16
    arch::x86_64::_mm_slli_epi32
    arch::x86_64::_mm_slli_epi64
    arch::x86_64::_mm_slli_si128
    arch::x86_64::_mm_sllv_epi32
    arch::x86_64::_mm_sllv_epi64
    arch::x86_64::_mm_sqrt_pd
    arch::x86_64::_mm_sqrt_ps
    arch::x86_64::_mm_sqrt_sd
    arch::x86_64::_mm_sqrt_ss
    arch::x86_64::_mm_sra_epi16
    arch::x86_64::_mm_sra_epi32
    arch::x86_64::_mm_srai_epi16
    arch::x86_64::_mm_srai_epi32
    arch::x86_64::_mm_srav_epi32
    arch::x86_64::_mm_srl_epi16
    arch::x86_64::_mm_srl_epi32
    arch::x86_64::_mm_srl_epi64
    arch::x86_64::_mm_srli_epi16
    arch::x86_64::_mm_srli_epi32
    arch::x86_64::_mm_srli_epi64
    arch::x86_64::_mm_srli_si128
    arch::x86_64::_mm_srlv_epi32
    arch::x86_64::_mm_srlv_epi64
    arch::x86_64::_mm_store1_pd
    arch::x86_64::_mm_store1_ps
    arch::x86_64::_mm_store_pd
    arch::x86_64::_mm_store_pd1
    arch::x86_64::_mm_store_ps
    arch::x86_64::_mm_store_ps1
    arch::x86_64::_mm_store_sd
    arch::x86_64::_mm_store_si128
    arch::x86_64::_mm_store_ss
    arch::x86_64::_mm_storeh_pd
    arch::x86_64::_mm_storeh_pi
    arch::x86_64::_mm_storel_epi64
    arch::x86_64::_mm_storel_pd
    arch::x86_64::_mm_storel_pi
    arch::x86_64::_mm_storer_pd
    arch::x86_64::_mm_storer_ps
    arch::x86_64::_mm_storeu_pd
    arch::x86_64::_mm_storeu_ps
    arch::x86_64::_mm_storeu_si128
    arch::x86_64::_mm_stream_pd
    arch::x86_64::_mm_stream_pi
    arch::x86_64::_mm_stream_ps
    arch::x86_64::_mm_stream_sd
    arch::x86_64::_mm_stream_si128
    arch::x86_64::_mm_stream_si32
    arch::x86_64::_mm_stream_si64
    arch::x86_64::_mm_stream_ss
    arch::x86_64::_mm_sub_epi16
    arch::x86_64::_mm_sub_epi32
    arch::x86_64::_mm_sub_epi64
    arch::x86_64::_mm_sub_epi8
    arch::x86_64::_mm_sub_pd
    arch::x86_64::_mm_sub_pi16
    arch::x86_64::_mm_sub_pi32
    arch::x86_64::_mm_sub_pi8
    arch::x86_64::_mm_sub_ps
    arch::x86_64::_mm_sub_sd
    arch::x86_64::_mm_sub_si64
    arch::x86_64::_mm_sub_ss
    arch::x86_64::_mm_subs_epi16
    arch::x86_64::_mm_subs_epi8
    arch::x86_64::_mm_subs_epu16
    arch::x86_64::_mm_subs_epu8
    arch::x86_64::_mm_subs_pi16
    arch::x86_64::_mm_subs_pi8
    arch::x86_64::_mm_subs_pu16
    arch::x86_64::_mm_subs_pu8
    arch::x86_64::_mm_test_all_ones
    arch::x86_64::_mm_test_all_zeros
    arch::x86_64::_mm_test_mix_ones_zeros
    arch::x86_64::_mm_testc_pd
    arch::x86_64::_mm_testc_ps
    arch::x86_64::_mm_testc_si128
    arch::x86_64::_mm_testnzc_pd
    arch::x86_64::_mm_testnzc_ps
    arch::x86_64::_mm_testnzc_si128
    arch::x86_64::_mm_testz_pd
    arch::x86_64::_mm_testz_ps
    arch::x86_64::_mm_testz_si128
    arch::x86_64::_mm_tzcnt_32
    arch::x86_64::_mm_tzcnt_64
    arch::x86_64::_mm_ucomieq_sd
    arch::x86_64::_mm_ucomieq_ss
    arch::x86_64::_mm_ucomige_sd
    arch::x86_64::_mm_ucomige_ss
    arch::x86_64::_mm_ucomigt_sd
    arch::x86_64::_mm_ucomigt_ss
    arch::x86_64::_mm_ucomile_sd
    arch::x86_64::_mm_ucomile_ss
    arch::x86_64::_mm_ucomilt_sd
    arch::x86_64::_mm_ucomilt_ss
    arch::x86_64::_mm_ucomineq_sd
    arch::x86_64::_mm_ucomineq_ss
    arch::x86_64::_mm_undefined_pd
    arch::x86_64::_mm_undefined_ps
    arch::x86_64::_mm_undefined_si128
    arch::x86_64::_mm_unpackhi_epi16
    arch::x86_64::_mm_unpackhi_epi32
    arch::x86_64::_mm_unpackhi_epi64
    arch::x86_64::_mm_unpackhi_epi8
    arch::x86_64::_mm_unpackhi_pd
    arch::x86_64::_mm_unpackhi_pi16
    arch::x86_64::_mm_unpackhi_pi32
    arch::x86_64::_mm_unpackhi_pi8
    arch::x86_64::_mm_unpackhi_ps
    arch::x86_64::_mm_unpacklo_epi16
    arch::x86_64::_mm_unpacklo_epi32
    arch::x86_64::_mm_unpacklo_epi64
    arch::x86_64::_mm_unpacklo_epi8
    arch::x86_64::_mm_unpacklo_pd
    arch::x86_64::_mm_unpacklo_pi16
    arch::x86_64::_mm_unpacklo_pi32
    arch::x86_64::_mm_unpacklo_pi8
    arch::x86_64::_mm_unpacklo_ps
    arch::x86_64::_mm_xor_pd
    arch::x86_64::_mm_xor_ps
    arch::x86_64::_mm_xor_si128
    arch::x86_64::_mulx_u32
    arch::x86_64::_mulx_u64
    arch::x86_64::_pdep_u32
    arch::x86_64::_pdep_u64
    arch::x86_64::_pext_u32
    arch::x86_64::_pext_u64
    arch::x86_64::_popcnt32
    arch::x86_64::_popcnt64
    arch::x86_64::_rdrand16_step
    arch::x86_64::_rdrand32_step
    arch::x86_64::_rdrand64_step
    arch::x86_64::_rdseed16_step
    arch::x86_64::_rdseed32_step
    arch::x86_64::_rdseed64_step
    arch::x86_64::_rdtsc
    arch::x86_64::_t1mskc_u32
    arch::x86_64::_t1mskc_u64
    arch::x86_64::_tzcnt_u32
    arch::x86_64::_tzcnt_u64
    arch::x86_64::_tzmsk_u32
    arch::x86_64::_tzmsk_u64
    arch::x86_64::_xgetbv
    arch::x86_64::_xrstor
    arch::x86_64::_xrstor64
    arch::x86_64::_xrstors
    arch::x86_64::_xrstors64
    arch::x86_64::_xsave
    arch::x86_64::_xsave64
    arch::x86_64::_xsavec
    arch::x86_64::_xsavec64
    arch::x86_64::_xsaveopt
    arch::x86_64::_xsaveopt64
    arch::x86_64::_xsaves
    arch::x86_64::_xsaves64
    arch::x86_64::_xsetbv
    arch::x86_64::has_cpuid
  ascii::escape_default
  char::decode_utf16
  char::decode_utf8
  char::from_digit
  char::from_u32
  char::from_u32_unchecked
  cmp::max
  cmp::min
  fmt::write
  hint::unreachable_unchecked
  intrinsics
    intrinsics::abort
    intrinsics::add_with_overflow
    intrinsics::arith_offset
    intrinsics::assume
    intrinsics::atomic_and
    intrinsics::atomic_and_acq
    intrinsics::atomic_and_acqrel
    intrinsics::atomic_and_rel
    intrinsics::atomic_and_relaxed
    intrinsics::atomic_cxchg
    intrinsics::atomic_cxchg_acq
    intrinsics::atomic_cxchg_acq_failrelaxed
    intrinsics::atomic_cxchg_acqrel
    intrinsics::atomic_cxchg_acqrel_failrelaxed
    intrinsics::atomic_cxchg_failacq
    intrinsics::atomic_cxchg_failrelaxed
    intrinsics::atomic_cxchg_rel
    intrinsics::atomic_cxchg_relaxed
    intrinsics::atomic_cxchgweak
    intrinsics::atomic_cxchgweak_acq
    intrinsics::atomic_cxchgweak_acq_failrelaxed
    intrinsics::atomic_cxchgweak_acqrel
    intrinsics::atomic_cxchgweak_acqrel_failrelaxed
    intrinsics::atomic_cxchgweak_failacq
    intrinsics::atomic_cxchgweak_failrelaxed
    intrinsics::atomic_cxchgweak_rel
    intrinsics::atomic_cxchgweak_relaxed
    intrinsics::atomic_fence
    intrinsics::atomic_fence_acq
    intrinsics::atomic_fence_acqrel
    intrinsics::atomic_fence_rel
    intrinsics::atomic_load
    intrinsics::atomic_load_acq
    intrinsics::atomic_load_relaxed
    intrinsics::atomic_load_unordered
    intrinsics::atomic_max
    intrinsics::atomic_max_acq
    intrinsics::atomic_max_acqrel
    intrinsics::atomic_max_rel
    intrinsics::atomic_max_relaxed
    intrinsics::atomic_min
    intrinsics::atomic_min_acq
    intrinsics::atomic_min_acqrel
    intrinsics::atomic_min_rel
    intrinsics::atomic_min_relaxed
    intrinsics::atomic_nand
    intrinsics::atomic_nand_acq
    intrinsics::atomic_nand_acqrel
    intrinsics::atomic_nand_rel
    intrinsics::atomic_nand_relaxed
    intrinsics::atomic_or
    intrinsics::atomic_or_acq
    intrinsics::atomic_or_acqrel
    intrinsics::atomic_or_rel
    intrinsics::atomic_or_relaxed
    intrinsics::atomic_singlethreadfence
    intrinsics::atomic_singlethreadfence_acq
    intrinsics::atomic_singlethreadfence_acqrel
    intrinsics::atomic_singlethreadfence_rel
    intrinsics::atomic_store
    intrinsics::atomic_store_rel
    intrinsics::atomic_store_relaxed
    intrinsics::atomic_store_unordered
    intrinsics::atomic_umax
    intrinsics::atomic_umax_acq
    intrinsics::atomic_umax_acqrel
    intrinsics::atomic_umax_rel
    intrinsics::atomic_umax_relaxed
    intrinsics::atomic_umin
    intrinsics::atomic_umin_acq
    intrinsics::atomic_umin_acqrel
    intrinsics::atomic_umin_rel
    intrinsics::atomic_umin_relaxed
    intrinsics::atomic_xadd
    intrinsics::atomic_xadd_acq
    intrinsics::atomic_xadd_acqrel
    intrinsics::atomic_xadd_rel
    intrinsics::atomic_xadd_relaxed
    intrinsics::atomic_xchg
    intrinsics::atomic_xchg_acq
    intrinsics::atomic_xchg_acqrel
    intrinsics::atomic_xchg_rel
    intrinsics::atomic_xchg_relaxed
    intrinsics::atomic_xor
    intrinsics::atomic_xor_acq
    intrinsics::atomic_xor_acqrel
    intrinsics::atomic_xor_rel
    intrinsics::atomic_xor_relaxed
    intrinsics::atomic_xsub
    intrinsics::atomic_xsub_acq
    intrinsics::atomic_xsub_acqrel
    intrinsics::atomic_xsub_rel
    intrinsics::atomic_xsub_relaxed
    intrinsics::bitreverse
    intrinsics::breakpoint
    intrinsics::bswap
    intrinsics::ceilf32
    intrinsics::ceilf64
    intrinsics::copy
    intrinsics::copy_nonoverlapping
    intrinsics::copysignf32
    intrinsics::copysignf64
    intrinsics::cosf32
    intrinsics::cosf64
    intrinsics::ctlz
    intrinsics::ctlz_nonzero
    intrinsics::ctpop
    intrinsics::cttz
    intrinsics::cttz_nonzero
    intrinsics::discriminant_value
    intrinsics::exact_div
    intrinsics::exp2f32
    intrinsics::exp2f64
    intrinsics::expf32
    intrinsics::expf64
    intrinsics::fabsf32
    intrinsics::fabsf64
    intrinsics::fadd_fast
    intrinsics::fdiv_fast
    intrinsics::floorf32
    intrinsics::floorf64
    intrinsics::fmaf32
    intrinsics::fmaf64
    intrinsics::fmul_fast
    intrinsics::frem_fast
    intrinsics::fsub_fast
    intrinsics::init
    intrinsics::likely
    intrinsics::log10f32
    intrinsics::log10f64
    intrinsics::log2f32
    intrinsics::log2f64
    intrinsics::logf32
    intrinsics::logf64
    intrinsics::min_align_of
    intrinsics::min_align_of_val
    intrinsics::move_val_init
    intrinsics::mul_with_overflow
    intrinsics::nearbyintf32
    intrinsics::nearbyintf64
    intrinsics::needs_drop
    intrinsics::nontemporal_store
    intrinsics::offset
    intrinsics::overflowing_add
    intrinsics::overflowing_mul
    intrinsics::overflowing_sub
    intrinsics::powf32
    intrinsics::powf64
    intrinsics::powif32
    intrinsics::powif64
    intrinsics::pref_align_of
    intrinsics::prefetch_read_data
    intrinsics::prefetch_read_instruction
    intrinsics::prefetch_write_data
    intrinsics::prefetch_write_instruction
    intrinsics::rintf32
    intrinsics::rintf64
    intrinsics::roundf32
    intrinsics::roundf64
    intrinsics::rustc_peek
    intrinsics::sinf32
    intrinsics::sinf64
    intrinsics::size_of
    intrinsics::size_of_val
    intrinsics::sqrtf32
    intrinsics::sqrtf64
    intrinsics::sub_with_overflow
    intrinsics::transmute
    intrinsics::truncf32
    intrinsics::truncf64
    intrinsics::try
    intrinsics::type_id
    intrinsics::type_name
    intrinsics::unchecked_div
    intrinsics::unchecked_rem
    intrinsics::unchecked_shl
    intrinsics::unchecked_shr
    intrinsics::uninit
    intrinsics::unlikely
    intrinsics::unreachable
    intrinsics::volatile_copy_memory
    intrinsics::volatile_copy_nonoverlapping_memory
    intrinsics::volatile_load
    intrinsics::volatile_set_memory
    intrinsics::volatile_store
    intrinsics::write_bytes
  iter::empty
  iter::once
  iter::repeat
  iter::repeat_with
  mem::align_of
  mem::align_of_val
  mem::discriminant
  mem::drop
  mem::forget
  mem::min_align_of
  mem::min_align_of_val
  mem::needs_drop
  mem::replace
  mem::size_of
  mem::size_of_val
  mem::swap
  mem::transmute_copy
  mem::uninitialized
  mem::zeroed
  panicking::panic
  panicking::panic_fmt
  ptr::drop_in_place
  ptr::eq
  ptr::null
  ptr::null_mut
  ptr::read
  ptr::read_unaligned
  ptr::read_volatile
  ptr::replace
  ptr::swap
  ptr::swap_nonoverlapping
  ptr::write
  ptr::write_unaligned
  ptr::write_volatile
  slice::from_mut
  slice::from_raw_parts
  slice::from_raw_parts_mut
  slice::from_ref
  slice::memchr::memchr
  slice::memchr::memrchr
  str::from_utf8
  str::from_utf8_mut
  str::from_utf8_unchecked
  str::from_utf8_unchecked_mut
  str::next_code_point
  str::utf8_char_width
  sync::atomic::compiler_fence
  sync::atomic::fence
  sync::atomic::spin_loop_hint
  unicode::derived_property::Case_Ignorable
  unicode::derived_property::Cased
  unicode::property::Pattern_White_Space

## Typedefs
  fmt::Result

## Constants
  - char::MAX
  - char::REPLACEMENT_CHARACTER
  - char::UNICODE_VERSION
  - f32
    f32::DIGITS
    f32::EPSILON
    f32::INFINITY
    f32::MANTISSA_DIGITS
    f32::MAX
    f32::MAX_10_EXP
    f32::MAX_EXP
    f32::MIN
    f32::MIN_10_EXP
    f32::MIN_EXP
    f32::MIN_POSITIVE
    f32::NAN
    f32::NEG_INFINITY
    f32::RADIX
    f32::consts::E
    f32::consts::FRAC_1_PI
    f32::consts::FRAC_1_SQRT_2
    f32::consts::FRAC_2_PI
    f32::consts::FRAC_2_SQRT_PI
    f32::consts::FRAC_PI_2
    f32::consts::FRAC_PI_3
    f32::consts::FRAC_PI_4
    f32::consts::FRAC_PI_6
    f32::consts::FRAC_PI_8
    f32::consts::LN_10
    f32::consts::LN_2
    f32::consts::LOG10_2
    f32::consts::LOG10_E
    f32::consts::LOG2_10
    f32::consts::LOG2_E
    f32::consts::PI
    f32::consts::SQRT_2
    
  - f64
    f64::DIGITS
    f64::EPSILON
    f64::INFINITY
    f64::MANTISSA_DIGITS
    f64::MAX
    f64::MAX_10_EXP
    f64::MAX_EXP
    f64::MIN
    f64::MIN_10_EXP
    f64::MIN_EXP
    f64::MIN_POSITIVE
    f64::NAN
    f64::NEG_INFINITY
    f64::RADIX
    f64::consts::E
    f64::consts::FRAC_1_PI
    f64::consts::FRAC_1_SQRT_2
    f64::consts::FRAC_2_PI
    f64::consts::FRAC_2_SQRT_PI
    f64::consts::FRAC_PI_2
    f64::consts::FRAC_PI_3
    f64::consts::FRAC_PI_4
    f64::consts::FRAC_PI_6
    f64::consts::FRAC_PI_8
    f64::consts::LN_10
    f64::consts::LN_2
    f64::consts::LOG10_2
    f64::consts::LOG10_E
    f64::consts::LOG2_10
    f64::consts::LOG2_E
    f64::consts::PI
    f64::consts::SQRT_2
  i128::MAX
  i128::MIN
  i16::MAX
  i16::MIN
  i32::MAX
  i32::MIN
  i64::MAX
  i64::MIN
  i8::MAX
  i8::MIN
  isize::MAX
  isize::MIN
  u128::MAX
  u128::MIN
  u16::MAX
  u16::MIN
  u32::MAX
  u32::MIN
  u64::MAX
  u64::MIN
  u8::MAX
  u8::MIN
  usize::MAX
  usize::MIN
  - sync
    sync::atomic::ATOMIC_BOOL_INIT
    sync::atomic::ATOMIC_I16_INIT
    sync::atomic::ATOMIC_I32_INIT
    sync::atomic::ATOMIC_I64_INIT
    sync::atomic::ATOMIC_I8_INIT
    sync::atomic::ATOMIC_ISIZE_INIT
    sync::atomic::ATOMIC_U16_INIT
    sync::atomic::ATOMIC_U32_INIT
    sync::atomic::ATOMIC_U64_INIT
    sync::atomic::ATOMIC_U8_INIT
    sync::atomic::ATOMIC_USIZE_INIT
  - arch
    arch::x86::_CMP_EQ_OQ
    arch::x86::_CMP_EQ_OS
    arch::x86::_CMP_EQ_UQ
    arch::x86::_CMP_EQ_US
    arch::x86::_CMP_FALSE_OQ
    arch::x86::_CMP_FALSE_OS
    arch::x86::_CMP_GE_OQ
    arch::x86::_CMP_GE_OS
    arch::x86::_CMP_GT_OQ
    arch::x86::_CMP_GT_OS
    arch::x86::_CMP_LE_OQ
    arch::x86::_CMP_LE_OS
    arch::x86::_CMP_LT_OQ
    arch::x86::_CMP_LT_OS
    arch::x86::_CMP_NEQ_OQ
    arch::x86::_CMP_NEQ_OS
    arch::x86::_CMP_NEQ_UQ
    arch::x86::_CMP_NEQ_US
    arch::x86::_CMP_NGE_UQ
    arch::x86::_CMP_NGE_US
    arch::x86::_CMP_NGT_UQ
    arch::x86::_CMP_NGT_US
    arch::x86::_CMP_NLE_UQ
    arch::x86::_CMP_NLE_US
    arch::x86::_CMP_NLT_UQ
    arch::x86::_CMP_NLT_US
    arch::x86::_CMP_ORD_Q
    arch::x86::_CMP_ORD_S
    arch::x86::_CMP_TRUE_UQ
    arch::x86::_CMP_TRUE_US
    arch::x86::_CMP_UNORD_Q
    arch::x86::_CMP_UNORD_S
    arch::x86::_MM_EXCEPT_DENORM
    arch::x86::_MM_EXCEPT_DIV_ZERO
    arch::x86::_MM_EXCEPT_INEXACT
    arch::x86::_MM_EXCEPT_INVALID
    arch::x86::_MM_EXCEPT_MASK
    arch::x86::_MM_EXCEPT_OVERFLOW
    arch::x86::_MM_EXCEPT_UNDERFLOW
    arch::x86::_MM_FLUSH_ZERO_MASK
    arch::x86::_MM_FLUSH_ZERO_OFF
    arch::x86::_MM_FLUSH_ZERO_ON
    arch::x86::_MM_FROUND_CEIL
    arch::x86::_MM_FROUND_CUR_DIRECTION
    arch::x86::_MM_FROUND_FLOOR
    arch::x86::_MM_FROUND_NEARBYINT
    arch::x86::_MM_FROUND_NINT
    arch::x86::_MM_FROUND_NO_EXC
    arch::x86::_MM_FROUND_RAISE_EXC
    arch::x86::_MM_FROUND_RINT
    arch::x86::_MM_FROUND_TO_NEAREST_INT
    arch::x86::_MM_FROUND_TO_NEG_INF
    arch::x86::_MM_FROUND_TO_POS_INF
    arch::x86::_MM_FROUND_TO_ZERO
    arch::x86::_MM_FROUND_TRUNC
    arch::x86::_MM_HINT_NTA
    arch::x86::_MM_HINT_T0
    arch::x86::_MM_HINT_T1
    arch::x86::_MM_HINT_T2
    arch::x86::_MM_MASK_DENORM
    arch::x86::_MM_MASK_DIV_ZERO
    arch::x86::_MM_MASK_INEXACT
    arch::x86::_MM_MASK_INVALID
    arch::x86::_MM_MASK_MASK
    arch::x86::_MM_MASK_OVERFLOW
    arch::x86::_MM_MASK_UNDERFLOW
    arch::x86::_MM_ROUND_DOWN
    arch::x86::_MM_ROUND_MASK
    arch::x86::_MM_ROUND_NEAREST
    arch::x86::_MM_ROUND_TOWARD_ZERO
    arch::x86::_MM_ROUND_UP
    arch::x86::_SIDD_BIT_MASK
    arch::x86::_SIDD_CMP_EQUAL_ANY
    arch::x86::_SIDD_CMP_EQUAL_EACH
    arch::x86::_SIDD_CMP_EQUAL_ORDERED
    arch::x86::_SIDD_CMP_RANGES
    arch::x86::_SIDD_LEAST_SIGNIFICANT
    arch::x86::_SIDD_MASKED_NEGATIVE_POLARITY
    arch::x86::_SIDD_MASKED_POSITIVE_POLARITY
    arch::x86::_SIDD_MOST_SIGNIFICANT
    arch::x86::_SIDD_NEGATIVE_POLARITY
    arch::x86::_SIDD_POSITIVE_POLARITY
    arch::x86::_SIDD_SBYTE_OPS
    arch::x86::_SIDD_SWORD_OPS
    arch::x86::_SIDD_UBYTE_OPS
    arch::x86::_SIDD_UNIT_MASK
    arch::x86::_SIDD_UWORD_OPS
    arch::x86::_XCR_XFEATURE_ENABLED_MASK
    arch::x86_64::_CMP_EQ_OQ
    arch::x86_64::_CMP_EQ_OS
    arch::x86_64::_CMP_EQ_UQ
    arch::x86_64::_CMP_EQ_US
    arch::x86_64::_CMP_FALSE_OQ
    arch::x86_64::_CMP_FALSE_OS
    arch::x86_64::_CMP_GE_OQ
    arch::x86_64::_CMP_GE_OS
    arch::x86_64::_CMP_GT_OQ
    arch::x86_64::_CMP_GT_OS
    arch::x86_64::_CMP_LE_OQ
    arch::x86_64::_CMP_LE_OS
    arch::x86_64::_CMP_LT_OQ
    arch::x86_64::_CMP_LT_OS
    arch::x86_64::_CMP_NEQ_OQ
    arch::x86_64::_CMP_NEQ_OS
    arch::x86_64::_CMP_NEQ_UQ
    arch::x86_64::_CMP_NEQ_US
    arch::x86_64::_CMP_NGE_UQ
    arch::x86_64::_CMP_NGE_US
    arch::x86_64::_CMP_NGT_UQ
    arch::x86_64::_CMP_NGT_US
    arch::x86_64::_CMP_NLE_UQ
    arch::x86_64::_CMP_NLE_US
    arch::x86_64::_CMP_NLT_UQ
    arch::x86_64::_CMP_NLT_US
    arch::x86_64::_CMP_ORD_Q
    arch::x86_64::_CMP_ORD_S
    arch::x86_64::_CMP_TRUE_UQ
    arch::x86_64::_CMP_TRUE_US
    arch::x86_64::_CMP_UNORD_Q
    arch::x86_64::_CMP_UNORD_S
    arch::x86_64::_MM_EXCEPT_DENORM
    arch::x86_64::_MM_EXCEPT_DIV_ZERO
    arch::x86_64::_MM_EXCEPT_INEXACT
    arch::x86_64::_MM_EXCEPT_INVALID
    arch::x86_64::_MM_EXCEPT_MASK
    arch::x86_64::_MM_EXCEPT_OVERFLOW
    arch::x86_64::_MM_EXCEPT_UNDERFLOW
    arch::x86_64::_MM_FLUSH_ZERO_MASK
    arch::x86_64::_MM_FLUSH_ZERO_OFF
    arch::x86_64::_MM_FLUSH_ZERO_ON
    arch::x86_64::_MM_FROUND_CEIL
    arch::x86_64::_MM_FROUND_CUR_DIRECTION
    arch::x86_64::_MM_FROUND_FLOOR
    arch::x86_64::_MM_FROUND_NEARBYINT
    arch::x86_64::_MM_FROUND_NINT
    arch::x86_64::_MM_FROUND_NO_EXC
    arch::x86_64::_MM_FROUND_RAISE_EXC
    arch::x86_64::_MM_FROUND_RINT
    arch::x86_64::_MM_FROUND_TO_NEAREST_INT
    arch::x86_64::_MM_FROUND_TO_NEG_INF
    arch::x86_64::_MM_FROUND_TO_POS_INF
    arch::x86_64::_MM_FROUND_TO_ZERO
    arch::x86_64::_MM_FROUND_TRUNC
    arch::x86_64::_MM_HINT_NTA
    arch::x86_64::_MM_HINT_T0
    arch::x86_64::_MM_HINT_T1
    arch::x86_64::_MM_HINT_T2
    arch::x86_64::_MM_MASK_DENORM
    arch::x86_64::_MM_MASK_DIV_ZERO
    arch::x86_64::_MM_MASK_INEXACT
    arch::x86_64::_MM_MASK_INVALID
    arch::x86_64::_MM_MASK_MASK
    arch::x86_64::_MM_MASK_OVERFLOW
    arch::x86_64::_MM_MASK_UNDERFLOW
    arch::x86_64::_MM_ROUND_DOWN
    arch::x86_64::_MM_ROUND_MASK
    arch::x86_64::_MM_ROUND_NEAREST
    arch::x86_64::_MM_ROUND_TOWARD_ZERO
    arch::x86_64::_MM_ROUND_UP
    arch::x86_64::_SIDD_BIT_MASK
    arch::x86_64::_SIDD_CMP_EQUAL_ANY
    arch::x86_64::_SIDD_CMP_EQUAL_EACH
    arch::x86_64::_SIDD_CMP_EQUAL_ORDERED
    arch::x86_64::_SIDD_CMP_RANGES
    arch::x86_64::_SIDD_LEAST_SIGNIFICANT
    arch::x86_64::_SIDD_MASKED_NEGATIVE_POLARITY
    arch::x86_64::_SIDD_MASKED_POSITIVE_POLARITY
    arch::x86_64::_SIDD_MOST_SIGNIFICANT
    arch::x86_64::_SIDD_NEGATIVE_POLARITY
    arch::x86_64::_SIDD_POSITIVE_POLARITY
    arch::x86_64::_SIDD_SBYTE_OPS
    arch::x86_64::_SIDD_SWORD_OPS
    arch::x86_64::_SIDD_UBYTE_OPS
    arch::x86_64::_SIDD_UNIT_MASK
    arch::x86_64::_SIDD_UWORD_OPS
    arch::x86_64::_XCR_XFEATURE_ENABLED_MASK




---


Collected from: [List of all items in this crate](https://doc.rust-lang.org/nightly/core/all.html)

- [alloc::AllocErr](https://doc.rust-lang.org/nightly/core/alloc/struct.AllocErr.html)
- [alloc::CannotReallocInPlace](https://doc.rust-lang.org/nightly/core/alloc/struct.CannotReallocInPlace.html)
- [alloc::Excess](https://doc.rust-lang.org/nightly/core/alloc/struct.Excess.html)
- [alloc::Layout](https://doc.rust-lang.org/nightly/core/alloc/struct.Layout.html)
- [alloc::LayoutErr](https://doc.rust-lang.org/nightly/core/alloc/struct.LayoutErr.html)
- [any::TypeId](https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html)
- [arch::aarch64::float32x2_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.float32x2_t.html)
- [arch::aarch64::float32x4_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.float32x4_t.html)
- [arch::aarch64::float64x1_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.float64x1_t.html)
- [arch::aarch64::float64x2_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.float64x2_t.html)
- [arch::aarch64::int16x4_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int16x4_t.html)
- [arch::aarch64::int16x8_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int16x8_t.html)
- [arch::aarch64::int32x2_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int32x2_t.html)
- [arch::aarch64::int32x4_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int32x4_t.html)
- [arch::aarch64::int64x1_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int64x1_t.html)
- [arch::aarch64::int64x2_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int64x2_t.html)
- [arch::aarch64::int8x16_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int8x16_t.html)
- [arch::aarch64::int8x8_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.int8x8_t.html)
- [arch::aarch64::poly16x4_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.poly16x4_t.html)
- [arch::aarch64::poly16x8_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.poly16x8_t.html)
- [arch::aarch64::poly8x16_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.poly8x16_t.html)
- [arch::aarch64::poly8x8_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.poly8x8_t.html)
- [arch::aarch64::uint16x4_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint16x4_t.html)
- [arch::aarch64::uint16x8_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint16x8_t.html)
- [arch::aarch64::uint32x2_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint32x2_t.html)
- [arch::aarch64::uint32x4_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint32x4_t.html)
- [arch::aarch64::uint64x1_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint64x1_t.html)
- [arch::aarch64::uint64x2_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint64x2_t.html)
- [arch::aarch64::uint8x16_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint8x16_t.html)
- [arch::aarch64::uint8x8_t](https://doc.rust-lang.org/nightly/core/arch/aarch64/struct.uint8x8_t.html)
- [arch::arm::float32x2_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.float32x2_t.html)
- [arch::arm::float32x4_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.float32x4_t.html)
- [arch::arm::int16x4_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int16x4_t.html)
- [arch::arm::int16x8_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int16x8_t.html)
- [arch::arm::int32x2_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int32x2_t.html)
- [arch::arm::int32x4_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int32x4_t.html)
- [arch::arm::int64x1_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int64x1_t.html)
- [arch::arm::int64x2_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int64x2_t.html)
- [arch::arm::int8x16_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int8x16_t.html)
- [arch::arm::int8x8_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.int8x8_t.html)
- [arch::arm::poly16x4_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.poly16x4_t.html)
- [arch::arm::poly16x8_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.poly16x8_t.html)
- [arch::arm::poly8x16_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.poly8x16_t.html)
- [arch::arm::poly8x8_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.poly8x8_t.html)
- [arch::arm::uint16x4_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint16x4_t.html)
- [arch::arm::uint16x8_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint16x8_t.html)
- [arch::arm::uint32x2_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint32x2_t.html)
- [arch::arm::uint32x4_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint32x4_t.html)
- [arch::arm::uint64x1_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint64x1_t.html)
- [arch::arm::uint64x2_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint64x2_t.html)
- [arch::arm::uint8x16_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint8x16_t.html)
- [arch::arm::uint8x8_t](https://doc.rust-lang.org/nightly/core/arch/arm/struct.uint8x8_t.html)
- [arch::x86::CpuidResult](https://doc.rust-lang.org/nightly/core/arch/x86/struct.CpuidResult.html)
- [arch::x86::__m128](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m128.html)
- [arch::x86::__m128d](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m128d.html)
- [arch::x86::__m128i](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m128i.html)
- [arch::x86::__m256](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m256.html)
- [arch::x86::__m256d](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m256d.html)
- [arch::x86::__m256i](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m256i.html)
- [arch::x86::__m64](https://doc.rust-lang.org/nightly/core/arch/x86/struct.__m64.html)
- [arch::x86_64::CpuidResult](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.CpuidResult.html)
- [arch::x86_64::__m128](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m128.html)
- [arch::x86_64::__m128d](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m128d.html)
- [arch::x86_64::__m128i](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m128i.html)
- [arch::x86_64::__m256](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m256.html)
- [arch::x86_64::__m256d](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m256d.html)
- [arch::x86_64::__m256i](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m256i.html)
- [arch::x86_64::__m64](https://doc.rust-lang.org/nightly/core/arch/x86_64/struct.__m64.html)
- [array::TryFromSliceError](https://doc.rust-lang.org/nightly/core/array/struct.TryFromSliceError.html)
- [ascii::EscapeDefault](https://doc.rust-lang.org/nightly/core/ascii/struct.EscapeDefault.html)
- [cell::BorrowError](https://doc.rust-lang.org/nightly/core/cell/struct.BorrowError.html)
- [cell::BorrowMutError](https://doc.rust-lang.org/nightly/core/cell/struct.BorrowMutError.html)
- [cell::Cell](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html)
- [cell::Ref](https://doc.rust-lang.org/nightly/core/cell/struct.Ref.html)
- [cell::RefCell](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html)
- [cell::RefMut](https://doc.rust-lang.org/nightly/core/cell/struct.RefMut.html)
- [cell::UnsafeCell](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html)
- [char::CharTryFromError](https://doc.rust-lang.org/nightly/core/char/struct.CharTryFromError.html)
- [char::DecodeUtf16](https://doc.rust-lang.org/nightly/core/char/struct.DecodeUtf16.html)
- [char::DecodeUtf16Error](https://doc.rust-lang.org/nightly/core/char/struct.DecodeUtf16Error.html)
- [char::DecodeUtf8](https://doc.rust-lang.org/nightly/core/char/struct.DecodeUtf8.html)
- [char::EscapeDebug](https://doc.rust-lang.org/nightly/core/char/struct.EscapeDebug.html)
- [char::EscapeDefault](https://doc.rust-lang.org/nightly/core/char/struct.EscapeDefault.html)
- [char::EscapeUnicode](https://doc.rust-lang.org/nightly/core/char/struct.EscapeUnicode.html)
- [char::InvalidSequence](https://doc.rust-lang.org/nightly/core/char/struct.InvalidSequence.html)
- [char::ParseCharError](https://doc.rust-lang.org/nightly/core/char/struct.ParseCharError.html)
- [char::ToLowercase](https://doc.rust-lang.org/nightly/core/char/struct.ToLowercase.html)
- [char::ToUppercase](https://doc.rust-lang.org/nightly/core/char/struct.ToUppercase.html)
- [char::UnicodeVersion](https://doc.rust-lang.org/nightly/core/char/struct.UnicodeVersion.html)
- [cmp::Reverse](https://doc.rust-lang.org/nightly/core/cmp/struct.Reverse.html)
- [fmt::Arguments](https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html)
- [fmt::DebugList](https://doc.rust-lang.org/nightly/core/fmt/struct.DebugList.html)
- [fmt::DebugMap](https://doc.rust-lang.org/nightly/core/fmt/struct.DebugMap.html)
- [fmt::DebugSet](https://doc.rust-lang.org/nightly/core/fmt/struct.DebugSet.html)
- [fmt::DebugStruct](https://doc.rust-lang.org/nightly/core/fmt/struct.DebugStruct.html)
- [fmt::DebugTuple](https://doc.rust-lang.org/nightly/core/fmt/struct.DebugTuple.html)
- [fmt::Error](https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html)
- [fmt::Formatter](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html)
- [hash::BuildHasherDefault](https://doc.rust-lang.org/nightly/core/hash/struct.BuildHasherDefault.html)
- [hash::SipHasher](https://doc.rust-lang.org/nightly/core/hash/struct.SipHasher.html)
- [iter::Chain](https://doc.rust-lang.org/nightly/core/iter/struct.Chain.html)
- [iter::Cloned](https://doc.rust-lang.org/nightly/core/iter/struct.Cloned.html)
- [iter::Cycle](https://doc.rust-lang.org/nightly/core/iter/struct.Cycle.html)
- [iter::Empty](https://doc.rust-lang.org/nightly/core/iter/struct.Empty.html)
- [iter::Enumerate](https://doc.rust-lang.org/nightly/core/iter/struct.Enumerate.html)
- [iter::Filter](https://doc.rust-lang.org/nightly/core/iter/struct.Filter.html)
- [iter::FilterMap](https://doc.rust-lang.org/nightly/core/iter/struct.FilterMap.html)
- [iter::FlatMap](https://doc.rust-lang.org/nightly/core/iter/struct.FlatMap.html)
- [iter::Flatten](https://doc.rust-lang.org/nightly/core/iter/struct.Flatten.html)
- [iter::Fuse](https://doc.rust-lang.org/nightly/core/iter/struct.Fuse.html)
- [iter::Inspect](https://doc.rust-lang.org/nightly/core/iter/struct.Inspect.html)
- [iter::Map](https://doc.rust-lang.org/nightly/core/iter/struct.Map.html)
- [iter::Once](https://doc.rust-lang.org/nightly/core/iter/struct.Once.html)
- [iter::Peekable](https://doc.rust-lang.org/nightly/core/iter/struct.Peekable.html)
- [iter::Repeat](https://doc.rust-lang.org/nightly/core/iter/struct.Repeat.html)
- [iter::RepeatWith](https://doc.rust-lang.org/nightly/core/iter/struct.RepeatWith.html)
- [iter::Rev](https://doc.rust-lang.org/nightly/core/iter/struct.Rev.html)
- [iter::Scan](https://doc.rust-lang.org/nightly/core/iter/struct.Scan.html)
- [iter::Skip](https://doc.rust-lang.org/nightly/core/iter/struct.Skip.html)
- [iter::SkipWhile](https://doc.rust-lang.org/nightly/core/iter/struct.SkipWhile.html)
- [iter::StepBy](https://doc.rust-lang.org/nightly/core/iter/struct.StepBy.html)
- [iter::Take](https://doc.rust-lang.org/nightly/core/iter/struct.Take.html)
- [iter::TakeWhile](https://doc.rust-lang.org/nightly/core/iter/struct.TakeWhile.html)
- [iter::Zip](https://doc.rust-lang.org/nightly/core/iter/struct.Zip.html)
- [marker::PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html)
- [marker::Pinned](https://doc.rust-lang.org/nightly/core/marker/struct.Pinned.html)
- [mem::Discriminant](https://doc.rust-lang.org/nightly/core/mem/struct.Discriminant.html)
- [mem::PinMut](https://doc.rust-lang.org/nightly/core/mem/struct.PinMut.html)
- [num::NonZeroU128](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU128.html)
- [num::NonZeroU16](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU16.html)
- [num::NonZeroU32](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU32.html)
- [num::NonZeroU64](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU64.html)
- [num::NonZeroU8](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroU8.html)
- [num::NonZeroUsize](https://doc.rust-lang.org/nightly/core/num/struct.NonZeroUsize.html)
- [num::ParseFloatError](https://doc.rust-lang.org/nightly/core/num/struct.ParseFloatError.html)
- [num::ParseIntError](https://doc.rust-lang.org/nightly/core/num/struct.ParseIntError.html)
- [num::TryFromIntError](https://doc.rust-lang.org/nightly/core/num/struct.TryFromIntError.html)
- [num::Wrapping](https://doc.rust-lang.org/nightly/core/num/struct.Wrapping.html)
- [ops::Range](https://doc.rust-lang.org/nightly/core/ops/struct.Range.html)
- [ops::RangeFrom](https://doc.rust-lang.org/nightly/core/ops/struct.RangeFrom.html)
- [ops::RangeFull](https://doc.rust-lang.org/nightly/core/ops/struct.RangeFull.html)
- [ops::RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/struct.RangeInclusive.html)
- [ops::RangeTo](https://doc.rust-lang.org/nightly/core/ops/struct.RangeTo.html)
- [ops::RangeToInclusive](https://doc.rust-lang.org/nightly/core/ops/struct.RangeToInclusive.html)
- [option::IntoIter](https://doc.rust-lang.org/nightly/core/option/struct.IntoIter.html)
- [option::Iter](https://doc.rust-lang.org/nightly/core/option/struct.Iter.html)
- [option::IterMut](https://doc.rust-lang.org/nightly/core/option/struct.IterMut.html)
- [option::NoneError](https://doc.rust-lang.org/nightly/core/option/struct.NoneError.html)
- [panic::Location](https://doc.rust-lang.org/nightly/core/panic/struct.Location.html)
- [panic::PanicInfo](https://doc.rust-lang.org/nightly/core/panic/struct.PanicInfo.html)
- [ptr::NonNull](https://doc.rust-lang.org/nightly/core/ptr/struct.NonNull.html)
- [raw::TraitObject](https://doc.rust-lang.org/nightly/core/raw/struct.TraitObject.html)
- [result::IntoIter](https://doc.rust-lang.org/nightly/core/result/struct.IntoIter.html)
- [result::Iter](https://doc.rust-lang.org/nightly/core/result/struct.Iter.html)
- [result::IterMut](https://doc.rust-lang.org/nightly/core/result/struct.IterMut.html)
- [simd::f32x16](https://doc.rust-lang.org/nightly/core/simd/struct.f32x16.html)
- [simd::f32x2](https://doc.rust-lang.org/nightly/core/simd/struct.f32x2.html)
- [simd::f32x4](https://doc.rust-lang.org/nightly/core/simd/struct.f32x4.html)
- [simd::f32x8](https://doc.rust-lang.org/nightly/core/simd/struct.f32x8.html)
- [simd::f64x2](https://doc.rust-lang.org/nightly/core/simd/struct.f64x2.html)
- [simd::f64x4](https://doc.rust-lang.org/nightly/core/simd/struct.f64x4.html)
- [simd::f64x8](https://doc.rust-lang.org/nightly/core/simd/struct.f64x8.html)
- [simd::i16x16](https://doc.rust-lang.org/nightly/core/simd/struct.i16x16.html)
- [simd::i16x2](https://doc.rust-lang.org/nightly/core/simd/struct.i16x2.html)
- [simd::i16x32](https://doc.rust-lang.org/nightly/core/simd/struct.i16x32.html)
- [simd::i16x4](https://doc.rust-lang.org/nightly/core/simd/struct.i16x4.html)
- [simd::i16x8](https://doc.rust-lang.org/nightly/core/simd/struct.i16x8.html)
- [simd::i32x16](https://doc.rust-lang.org/nightly/core/simd/struct.i32x16.html)
- [simd::i32x2](https://doc.rust-lang.org/nightly/core/simd/struct.i32x2.html)
- [simd::i32x4](https://doc.rust-lang.org/nightly/core/simd/struct.i32x4.html)
- [simd::i32x8](https://doc.rust-lang.org/nightly/core/simd/struct.i32x8.html)
- [simd::i64x2](https://doc.rust-lang.org/nightly/core/simd/struct.i64x2.html)
- [simd::i64x4](https://doc.rust-lang.org/nightly/core/simd/struct.i64x4.html)
- [simd::i64x8](https://doc.rust-lang.org/nightly/core/simd/struct.i64x8.html)
- [simd::i8x16](https://doc.rust-lang.org/nightly/core/simd/struct.i8x16.html)
- [simd::i8x2](https://doc.rust-lang.org/nightly/core/simd/struct.i8x2.html)
- [simd::i8x32](https://doc.rust-lang.org/nightly/core/simd/struct.i8x32.html)
- [simd::i8x4](https://doc.rust-lang.org/nightly/core/simd/struct.i8x4.html)
- [simd::i8x64](https://doc.rust-lang.org/nightly/core/simd/struct.i8x64.html)
- [simd::i8x8](https://doc.rust-lang.org/nightly/core/simd/struct.i8x8.html)
- [simd::m16x16](https://doc.rust-lang.org/nightly/core/simd/struct.m16x16.html)
- [simd::m16x2](https://doc.rust-lang.org/nightly/core/simd/struct.m16x2.html)
- [simd::m16x4](https://doc.rust-lang.org/nightly/core/simd/struct.m16x4.html)
- [simd::m16x8](https://doc.rust-lang.org/nightly/core/simd/struct.m16x8.html)
- [simd::m1x16](https://doc.rust-lang.org/nightly/core/simd/struct.m1x16.html)
- [simd::m1x32](https://doc.rust-lang.org/nightly/core/simd/struct.m1x32.html)
- [simd::m1x64](https://doc.rust-lang.org/nightly/core/simd/struct.m1x64.html)
- [simd::m1x8](https://doc.rust-lang.org/nightly/core/simd/struct.m1x8.html)
- [simd::m32x2](https://doc.rust-lang.org/nightly/core/simd/struct.m32x2.html)
- [simd::m32x4](https://doc.rust-lang.org/nightly/core/simd/struct.m32x4.html)
- [simd::m32x8](https://doc.rust-lang.org/nightly/core/simd/struct.m32x8.html)
- [simd::m64x2](https://doc.rust-lang.org/nightly/core/simd/struct.m64x2.html)
- [simd::m64x4](https://doc.rust-lang.org/nightly/core/simd/struct.m64x4.html)
- [simd::m8x16](https://doc.rust-lang.org/nightly/core/simd/struct.m8x16.html)
- [simd::m8x2](https://doc.rust-lang.org/nightly/core/simd/struct.m8x2.html)
- [simd::m8x32](https://doc.rust-lang.org/nightly/core/simd/struct.m8x32.html)
- [simd::m8x4](https://doc.rust-lang.org/nightly/core/simd/struct.m8x4.html)
- [simd::m8x8](https://doc.rust-lang.org/nightly/core/simd/struct.m8x8.html)
- [simd::u16x16](https://doc.rust-lang.org/nightly/core/simd/struct.u16x16.html)
- [simd::u16x2](https://doc.rust-lang.org/nightly/core/simd/struct.u16x2.html)
- [simd::u16x32](https://doc.rust-lang.org/nightly/core/simd/struct.u16x32.html)
- [simd::u16x4](https://doc.rust-lang.org/nightly/core/simd/struct.u16x4.html)
- [simd::u16x8](https://doc.rust-lang.org/nightly/core/simd/struct.u16x8.html)
- [simd::u32x16](https://doc.rust-lang.org/nightly/core/simd/struct.u32x16.html)
- [simd::u32x2](https://doc.rust-lang.org/nightly/core/simd/struct.u32x2.html)
- [simd::u32x4](https://doc.rust-lang.org/nightly/core/simd/struct.u32x4.html)
- [simd::u32x8](https://doc.rust-lang.org/nightly/core/simd/struct.u32x8.html)
- [simd::u64x2](https://doc.rust-lang.org/nightly/core/simd/struct.u64x2.html)
- [simd::u64x4](https://doc.rust-lang.org/nightly/core/simd/struct.u64x4.html)
- [simd::u64x8](https://doc.rust-lang.org/nightly/core/simd/struct.u64x8.html)
- [simd::u8x16](https://doc.rust-lang.org/nightly/core/simd/struct.u8x16.html)
- [simd::u8x2](https://doc.rust-lang.org/nightly/core/simd/struct.u8x2.html)
- [simd::u8x32](https://doc.rust-lang.org/nightly/core/simd/struct.u8x32.html)
- [simd::u8x4](https://doc.rust-lang.org/nightly/core/simd/struct.u8x4.html)
- [simd::u8x64](https://doc.rust-lang.org/nightly/core/simd/struct.u8x64.html)
- [simd::u8x8](https://doc.rust-lang.org/nightly/core/simd/struct.u8x8.html)
- [slice::Chunks](https://doc.rust-lang.org/nightly/core/slice/struct.Chunks.html)
- [slice::ChunksMut](https://doc.rust-lang.org/nightly/core/slice/struct.ChunksMut.html)
- [slice::ExactChunks](https://doc.rust-lang.org/nightly/core/slice/struct.ExactChunks.html)
- [slice::ExactChunksMut](https://doc.rust-lang.org/nightly/core/slice/struct.ExactChunksMut.html)
- [slice::Iter](https://doc.rust-lang.org/nightly/core/slice/struct.Iter.html)
- [slice::IterMut](https://doc.rust-lang.org/nightly/core/slice/struct.IterMut.html)
- [slice::RSplit](https://doc.rust-lang.org/nightly/core/slice/struct.RSplit.html)
- [slice::RSplitMut](https://doc.rust-lang.org/nightly/core/slice/struct.RSplitMut.html)
- [slice::RSplitN](https://doc.rust-lang.org/nightly/core/slice/struct.RSplitN.html)
- [slice::RSplitNMut](https://doc.rust-lang.org/nightly/core/slice/struct.RSplitNMut.html)
- [slice::Split](https://doc.rust-lang.org/nightly/core/slice/struct.Split.html)
- [slice::SplitMut](https://doc.rust-lang.org/nightly/core/slice/struct.SplitMut.html)
- [slice::SplitN](https://doc.rust-lang.org/nightly/core/slice/struct.SplitN.html)
- [slice::SplitNMut](https://doc.rust-lang.org/nightly/core/slice/struct.SplitNMut.html)
- [slice::Windows](https://doc.rust-lang.org/nightly/core/slice/struct.Windows.html)
- [str::Bytes](https://doc.rust-lang.org/nightly/core/str/struct.Bytes.html)
- [str::CharIndices](https://doc.rust-lang.org/nightly/core/str/struct.CharIndices.html)
- [str::Chars](https://doc.rust-lang.org/nightly/core/str/struct.Chars.html)
- [str::EncodeUtf16](https://doc.rust-lang.org/nightly/core/str/struct.EncodeUtf16.html)
- [str::Lines](https://doc.rust-lang.org/nightly/core/str/struct.Lines.html)
- [str::LinesAny](https://doc.rust-lang.org/nightly/core/str/struct.LinesAny.html)
- [str::MatchIndices](https://doc.rust-lang.org/nightly/core/str/struct.MatchIndices.html)
- [str::Matches](https://doc.rust-lang.org/nightly/core/str/struct.Matches.html)
- [str::ParseBoolError](https://doc.rust-lang.org/nightly/core/str/struct.ParseBoolError.html)
- [str::RMatchIndices](https://doc.rust-lang.org/nightly/core/str/struct.RMatchIndices.html)
- [str::RMatches](https://doc.rust-lang.org/nightly/core/str/struct.RMatches.html)
- [str::RSplit](https://doc.rust-lang.org/nightly/core/str/struct.RSplit.html)
- [str::RSplitN](https://doc.rust-lang.org/nightly/core/str/struct.RSplitN.html)
- [str::RSplitTerminator](https://doc.rust-lang.org/nightly/core/str/struct.RSplitTerminator.html)
- [str::Split](https://doc.rust-lang.org/nightly/core/str/struct.Split.html)
- [str::SplitN](https://doc.rust-lang.org/nightly/core/str/struct.SplitN.html)
- [str::SplitTerminator](https://doc.rust-lang.org/nightly/core/str/struct.SplitTerminator.html)
- [str::SplitWhitespace](https://doc.rust-lang.org/nightly/core/str/struct.SplitWhitespace.html)
- [str::Utf8Error](https://doc.rust-lang.org/nightly/core/str/struct.Utf8Error.html)
- [str::lossy::Utf8Lossy](https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8Lossy.html)
- [str::lossy::Utf8LossyChunk](https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8LossyChunk.html)
- [str::lossy::Utf8LossyChunksIter](https://doc.rust-lang.org/nightly/core/str/lossy/struct.Utf8LossyChunksIter.html)
- [str::pattern::CharPredicateSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/struct.CharPredicateSearcher.html)
- [str::pattern::CharSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/struct.CharSearcher.html)
- [str::pattern::CharSliceSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/struct.CharSliceSearcher.html)
- [str::pattern::StrSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/struct.StrSearcher.html)
- [sync::atomic::AtomicBool](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicBool.html)
- [sync::atomic::AtomicI16](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicI16.html)
- [sync::atomic::AtomicI32](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicI32.html)
- [sync::atomic::AtomicI64](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicI64.html)
- [sync::atomic::AtomicI8](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicI8.html)
- [sync::atomic::AtomicIsize](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicIsize.html)
- [sync::atomic::AtomicPtr](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicPtr.html)
- [sync::atomic::AtomicU16](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicU16.html)
- [sync::atomic::AtomicU32](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicU32.html)
- [sync::atomic::AtomicU64](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicU64.html)
- [sync::atomic::AtomicU8](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicU8.html)
- [sync::atomic::AtomicUsize](https://doc.rust-lang.org/nightly/core/sync/atomic/struct.AtomicUsize.html)
- [time::Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html)
- [alloc::CollectionAllocErr](https://doc.rust-lang.org/nightly/core/alloc/enum.CollectionAllocErr.html)
- [cmp::Ordering](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html)
- [fmt::Alignment](https://doc.rust-lang.org/nightly/core/fmt/enum.Alignment.html)
- [num::FpCategory](https://doc.rust-lang.org/nightly/core/num/enum.FpCategory.html)
- [ops::Bound](https://doc.rust-lang.org/nightly/core/ops/enum.Bound.html)
- [ops::GeneratorState](https://doc.rust-lang.org/nightly/core/ops/enum.GeneratorState.html)
- [option::Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)
- [result::Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html)
- [str::pattern::SearchStep](https://doc.rust-lang.org/nightly/core/str/pattern/enum.SearchStep.html)
- [sync::atomic::Ordering](https://doc.rust-lang.org/nightly/core/sync/atomic/enum.Ordering.html)
- [mem::ManuallyDrop](https://doc.rust-lang.org/nightly/core/mem/union.ManuallyDrop.html)
- [alloc::Alloc](https://doc.rust-lang.org/nightly/core/alloc/trait.Alloc.html)
- [alloc::GlobalAlloc](https://doc.rust-lang.org/nightly/core/alloc/trait.GlobalAlloc.html)
- [any::Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html)
- [array::FixedSizeArray](https://doc.rust-lang.org/nightly/core/array/trait.FixedSizeArray.html)
- [borrow::Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html)
- [borrow::BorrowMut](https://doc.rust-lang.org/nightly/core/borrow/trait.BorrowMut.html)
- [clone::Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html)
- [cmp::Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html)
- [cmp::Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html)
- [cmp::PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)
- [cmp::PartialOrd](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html)
- [convert::AsMut](https://doc.rust-lang.org/nightly/core/convert/trait.AsMut.html)
- [convert::AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html)
- [convert::From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)
- [convert::Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html)
- [convert::TryFrom](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)
- [convert::TryInto](https://doc.rust-lang.org/nightly/core/convert/trait.TryInto.html)
- [default::Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html)
- [fmt::Binary](https://doc.rust-lang.org/nightly/core/fmt/trait.Binary.html)
- [fmt::Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html)
- [fmt::Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html)
- [fmt::LowerExp](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerExp.html)
- [fmt::LowerHex](https://doc.rust-lang.org/nightly/core/fmt/trait.LowerHex.html)
- [fmt::Octal](https://doc.rust-lang.org/nightly/core/fmt/trait.Octal.html)
- [fmt::Pointer](https://doc.rust-lang.org/nightly/core/fmt/trait.Pointer.html)
- [fmt::UpperExp](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperExp.html)
- [fmt::UpperHex](https://doc.rust-lang.org/nightly/core/fmt/trait.UpperHex.html)
- [fmt::Write](https://doc.rust-lang.org/nightly/core/fmt/trait.Write.html)
- [hash::BuildHasher](https://doc.rust-lang.org/nightly/core/hash/trait.BuildHasher.html)
- [hash::Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html)
- [hash::Hasher](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html)
- [iter::DoubleEndedIterator](https://doc.rust-lang.org/nightly/core/iter/trait.DoubleEndedIterator.html)
- [iter::ExactSizeIterator](https://doc.rust-lang.org/nightly/core/iter/trait.ExactSizeIterator.html)
- [iter::Extend](https://doc.rust-lang.org/nightly/core/iter/trait.Extend.html)
- [iter::FromIterator](https://doc.rust-lang.org/nightly/core/iter/trait.FromIterator.html)
- [iter::FusedIterator](https://doc.rust-lang.org/nightly/core/iter/trait.FusedIterator.html)
- [iter::IntoIterator](https://doc.rust-lang.org/nightly/core/iter/trait.IntoIterator.html)
- [iter::Iterator](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html)
- [iter::Product](https://doc.rust-lang.org/nightly/core/iter/trait.Product.html)
- [iter::Step](https://doc.rust-lang.org/nightly/core/iter/trait.Step.html)
- [iter::Sum](https://doc.rust-lang.org/nightly/core/iter/trait.Sum.html)
- [iter::TrustedLen](https://doc.rust-lang.org/nightly/core/iter/trait.TrustedLen.html)
- [marker::Copy](https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html)
- [marker::Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html)
- [marker::Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html)
- [marker::Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html)
- [marker::Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html)
- [marker::Unsize](https://doc.rust-lang.org/nightly/core/marker/trait.Unsize.html)
- [ops::Add](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html)
- [ops::AddAssign](https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html)
- [ops::BitAnd](https://doc.rust-lang.org/nightly/core/ops/trait.BitAnd.html)
- [ops::BitAndAssign](https://doc.rust-lang.org/nightly/core/ops/trait.BitAndAssign.html)
- [ops::BitOr](https://doc.rust-lang.org/nightly/core/ops/trait.BitOr.html)
- [ops::BitOrAssign](https://doc.rust-lang.org/nightly/core/ops/trait.BitOrAssign.html)
- [ops::BitXor](https://doc.rust-lang.org/nightly/core/ops/trait.BitXor.html)
- [ops::BitXorAssign](https://doc.rust-lang.org/nightly/core/ops/trait.BitXorAssign.html)
- [ops::CoerceUnsized](https://doc.rust-lang.org/nightly/core/ops/trait.CoerceUnsized.html)
- [ops::Deref](https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html)
- [ops::DerefMut](https://doc.rust-lang.org/nightly/core/ops/trait.DerefMut.html)
- [ops::Div](https://doc.rust-lang.org/nightly/core/ops/trait.Div.html)
- [ops::DivAssign](https://doc.rust-lang.org/nightly/core/ops/trait.DivAssign.html)
- [ops::Drop](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html)
- [ops::Fn](https://doc.rust-lang.org/nightly/core/ops/trait.Fn.html)
- [ops::FnMut](https://doc.rust-lang.org/nightly/core/ops/trait.FnMut.html)
- [ops::FnOnce](https://doc.rust-lang.org/nightly/core/ops/trait.FnOnce.html)
- [ops::Generator](https://doc.rust-lang.org/nightly/core/ops/trait.Generator.html)
- [ops::Index](https://doc.rust-lang.org/nightly/core/ops/trait.Index.html)
- [ops::IndexMut](https://doc.rust-lang.org/nightly/core/ops/trait.IndexMut.html)
- [ops::Mul](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html)
- [ops::MulAssign](https://doc.rust-lang.org/nightly/core/ops/trait.MulAssign.html)
- [ops::Neg](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html)
- [ops::Not](https://doc.rust-lang.org/nightly/core/ops/trait.Not.html)
- [ops::RangeBounds](https://doc.rust-lang.org/nightly/core/ops/trait.RangeBounds.html)
- [ops::Rem](https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html)
- [ops::RemAssign](https://doc.rust-lang.org/nightly/core/ops/trait.RemAssign.html)
- [ops::Shl](https://doc.rust-lang.org/nightly/core/ops/trait.Shl.html)
- [ops::ShlAssign](https://doc.rust-lang.org/nightly/core/ops/trait.ShlAssign.html)
- [ops::Shr](https://doc.rust-lang.org/nightly/core/ops/trait.Shr.html)
- [ops::ShrAssign](https://doc.rust-lang.org/nightly/core/ops/trait.ShrAssign.html)
- [ops::Sub](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html)
- [ops::SubAssign](https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html)
- [ops::Try](https://doc.rust-lang.org/nightly/core/ops/trait.Try.html)
- [simd::FromBits](https://doc.rust-lang.org/nightly/core/simd/trait.FromBits.html)
- [simd::IntoBits](https://doc.rust-lang.org/nightly/core/simd/trait.IntoBits.html)
- [slice::SliceIndex](https://doc.rust-lang.org/nightly/core/slice/trait.SliceIndex.html)
- [str::FromStr](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html)
- [str::pattern::DoubleEndedSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.DoubleEndedSearcher.html)
- [str::pattern::Pattern](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Pattern.html)
- [str::pattern::ReverseSearcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.ReverseSearcher.html)
- [str::pattern::Searcher](https://doc.rust-lang.org/nightly/core/str/pattern/trait.Searcher.html)
- [assert](https://doc.rust-lang.org/nightly/core/macro.assert.html)
- [assert_eq](https://doc.rust-lang.org/nightly/core/macro.assert_eq.html)
- [assert_ne](https://doc.rust-lang.org/nightly/core/macro.assert_ne.html)
- [https://doc.rust-lang.org/nightly/core/macro.cfg.h](https://doc.rust-lang.org/nightly/core/macro.cfg.html)
- [column](https://doc.rust-lang.org/nightly/core/macro.column.html)
- [compile_error](https://doc.rust-lang.org/nightly/core/macro.compile_error.html)
- [concat](https://doc.rust-lang.org/nightly/core/macro.concat.html)
- [concat_idents](https://doc.rust-lang.org/nightly/core/macro.concat_idents.html)
- [debug_assert](https://doc.rust-lang.org/nightly/core/macro.debug_assert.html)
- [debug_assert_eq](https://doc.rust-lang.org/nightly/core/macro.debug_assert_eq.html)
- [debug_assert_ne](https://doc.rust-lang.org/nightly/core/macro.debug_assert_ne.html)
- [https://doc.rust-lang.org/nightly/core/macro.env.h](https://doc.rust-lang.org/nightly/core/macro.env.html)
- [file](https://doc.rust-lang.org/nightly/core/macro.file.html)
- [format_args](https://doc.rust-lang.org/nightly/core/macro.format_args.html)
- [include](https://doc.rust-lang.org/nightly/core/macro.include.html)
- [include_bytes](https://doc.rust-lang.org/nightly/core/macro.include_bytes.html)
- [include_str](https://doc.rust-lang.org/nightly/core/macro.include_str.html)
- [line](https://doc.rust-lang.org/nightly/core/macro.line.html)
- [module_path](https://doc.rust-lang.org/nightly/core/macro.module_path.html)
- [option_env](https://doc.rust-lang.org/nightly/core/macro.option_env.html)
- [panic](https://doc.rust-lang.org/nightly/core/macro.panic.html)
- [stringify](https://doc.rust-lang.org/nightly/core/macro.stringify.html)
- [https://doc.rust-lang.org/nightly/core/macro.try.h](https://doc.rust-lang.org/nightly/core/macro.try.html)
- [unimplemented](https://doc.rust-lang.org/nightly/core/macro.unimplemented.html)
- [unreachable](https://doc.rust-lang.org/nightly/core/macro.unreachable.html)
- [write](https://doc.rust-lang.org/nightly/core/macro.write.html)
- [writeln](https://doc.rust-lang.org/nightly/core/macro.writeln.html)
- [arch::aarch64::_cls_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._cls_u32.html)
- [arch::aarch64::_cls_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._cls_u64.html)
- [arch::aarch64::_clz_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._clz_u64.html)
- [arch::aarch64::_rbit_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._rbit_u64.html)
- [arch::aarch64::_rev_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._rev_u16.html)
- [arch::aarch64::_rev_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._rev_u32.html)
- [arch::aarch64::_rev_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn._rev_u64.html)
- [arch::aarch64::vadd_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_f32.html)
- [arch::aarch64::vadd_f64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_f64.html)
- [arch::aarch64::vadd_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_s16.html)
- [arch::aarch64::vadd_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_s32.html)
- [arch::aarch64::vadd_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_s8.html)
- [arch::aarch64::vadd_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_u16.html)
- [arch::aarch64::vadd_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_u32.html)
- [arch::aarch64::vadd_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vadd_u8.html)
- [arch::aarch64::vaddd_s64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddd_s64.html)
- [arch::aarch64::vaddd_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddd_u64.html)
- [arch::aarch64::vaddl_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddl_s16.html)
- [arch::aarch64::vaddl_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddl_s32.html)
- [arch::aarch64::vaddl_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddl_s8.html)
- [arch::aarch64::vaddl_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddl_u16.html)
- [arch::aarch64::vaddl_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddl_u32.html)
- [arch::aarch64::vaddl_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddl_u8.html)
- [arch::aarch64::vaddq_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_f32.html)
- [arch::aarch64::vaddq_f64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_f64.html)
- [arch::aarch64::vaddq_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_s16.html)
- [arch::aarch64::vaddq_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_s32.html)
- [arch::aarch64::vaddq_s64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_s64.html)
- [arch::aarch64::vaddq_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_s8.html)
- [arch::aarch64::vaddq_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_u16.html)
- [arch::aarch64::vaddq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_u32.html)
- [arch::aarch64::vaddq_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_u64.html)
- [arch::aarch64::vaddq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaddq_u8.html)
- [arch::aarch64::vaesdq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaesdq_u8.html)
- [arch::aarch64::vaeseq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaeseq_u8.html)
- [arch::aarch64::vaesimcq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaesimcq_u8.html)
- [arch::aarch64::vaesmcq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vaesmcq_u8.html)
- [arch::aarch64::vmaxv_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_f32.html)
- [arch::aarch64::vmaxv_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_s16.html)
- [arch::aarch64::vmaxv_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_s32.html)
- [arch::aarch64::vmaxv_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_s8.html)
- [arch::aarch64::vmaxv_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_u16.html)
- [arch::aarch64::vmaxv_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_u32.html)
- [arch::aarch64::vmaxv_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxv_u8.html)
- [arch::aarch64::vmaxvq_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_f32.html)
- [arch::aarch64::vmaxvq_f64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_f64.html)
- [arch::aarch64::vmaxvq_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_s16.html)
- [arch::aarch64::vmaxvq_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_s32.html)
- [arch::aarch64::vmaxvq_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_s8.html)
- [arch::aarch64::vmaxvq_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_u16.html)
- [arch::aarch64::vmaxvq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_u32.html)
- [arch::aarch64::vmaxvq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmaxvq_u8.html)
- [arch::aarch64::vminv_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_f32.html)
- [arch::aarch64::vminv_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_s16.html)
- [arch::aarch64::vminv_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_s32.html)
- [arch::aarch64::vminv_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_s8.html)
- [arch::aarch64::vminv_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_u16.html)
- [arch::aarch64::vminv_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_u32.html)
- [arch::aarch64::vminv_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminv_u8.html)
- [arch::aarch64::vminvq_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_f32.html)
- [arch::aarch64::vminvq_f64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_f64.html)
- [arch::aarch64::vminvq_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_s16.html)
- [arch::aarch64::vminvq_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_s32.html)
- [arch::aarch64::vminvq_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_s8.html)
- [arch::aarch64::vminvq_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_u16.html)
- [arch::aarch64::vminvq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_u32.html)
- [arch::aarch64::vminvq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vminvq_u8.html)
- [arch::aarch64::vmovl_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovl_s16.html)
- [arch::aarch64::vmovl_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovl_s32.html)
- [arch::aarch64::vmovl_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovl_s8.html)
- [arch::aarch64::vmovl_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovl_u16.html)
- [arch::aarch64::vmovl_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovl_u32.html)
- [arch::aarch64::vmovl_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovl_u8.html)
- [arch::aarch64::vmovn_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovn_s16.html)
- [arch::aarch64::vmovn_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovn_s32.html)
- [arch::aarch64::vmovn_s64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovn_s64.html)
- [arch::aarch64::vmovn_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovn_u16.html)
- [arch::aarch64::vmovn_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovn_u32.html)
- [arch::aarch64::vmovn_u64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vmovn_u64.html)
- [arch::aarch64::vpmax_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_f32.html)
- [arch::aarch64::vpmax_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_s16.html)
- [arch::aarch64::vpmax_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_s32.html)
- [arch::aarch64::vpmax_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_s8.html)
- [arch::aarch64::vpmax_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_u16.html)
- [arch::aarch64::vpmax_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_u32.html)
- [arch::aarch64::vpmax_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmax_u8.html)
- [arch::aarch64::vpmaxq_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_f32.html)
- [arch::aarch64::vpmaxq_f64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_f64.html)
- [arch::aarch64::vpmaxq_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_s16.html)
- [arch::aarch64::vpmaxq_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_s32.html)
- [arch::aarch64::vpmaxq_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_s8.html)
- [arch::aarch64::vpmaxq_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_u16.html)
- [arch::aarch64::vpmaxq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_u32.html)
- [arch::aarch64::vpmaxq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmaxq_u8.html)
- [arch::aarch64::vpmin_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_f32.html)
- [arch::aarch64::vpmin_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_s16.html)
- [arch::aarch64::vpmin_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_s32.html)
- [arch::aarch64::vpmin_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_s8.html)
- [arch::aarch64::vpmin_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_u16.html)
- [arch::aarch64::vpmin_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_u32.html)
- [arch::aarch64::vpmin_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpmin_u8.html)
- [arch::aarch64::vpminq_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_f32.html)
- [arch::aarch64::vpminq_f64](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_f64.html)
- [arch::aarch64::vpminq_s16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_s16.html)
- [arch::aarch64::vpminq_s32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_s32.html)
- [arch::aarch64::vpminq_s8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_s8.html)
- [arch::aarch64::vpminq_u16](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_u16.html)
- [arch::aarch64::vpminq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_u32.html)
- [arch::aarch64::vpminq_u8](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vpminq_u8.html)
- [arch::aarch64::vrsqrte_f32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vrsqrte_f32.html)
- [arch::aarch64::vsha1cq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha1cq_u32.html)
- [arch::aarch64::vsha1h_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha1h_u32.html)
- [arch::aarch64::vsha1mq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha1mq_u32.html)
- [arch::aarch64::vsha1pq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha1pq_u32.html)
- [arch::aarch64::vsha1su0q_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha1su0q_u32.html)
- [arch::aarch64::vsha1su1q_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha1su1q_u32.html)
- [arch::aarch64::vsha256h2q_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha256h2q_u32.html)
- [arch::aarch64::vsha256hq_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha256hq_u32.html)
- [arch::aarch64::vsha256su0q_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha256su0q_u32.html)
- [arch::aarch64::vsha256su1q_u32](https://doc.rust-lang.org/nightly/core/arch/aarch64/fn.vsha256su1q_u32.html)
- [arch::arm::_rev_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn._rev_u16.html)
- [arch::arm::_rev_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn._rev_u32.html)
- [arch::arm::vadd_f32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_f32.html)
- [arch::arm::vadd_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_s16.html)
- [arch::arm::vadd_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_s32.html)
- [arch::arm::vadd_s8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_s8.html)
- [arch::arm::vadd_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_u16.html)
- [arch::arm::vadd_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_u32.html)
- [arch::arm::vadd_u8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vadd_u8.html)
- [arch::arm::vaddl_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddl_s16.html)
- [arch::arm::vaddl_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddl_s32.html)
- [arch::arm::vaddl_s8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddl_s8.html)
- [arch::arm::vaddl_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddl_u16.html)
- [arch::arm::vaddl_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddl_u32.html)
- [arch::arm::vaddl_u8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddl_u8.html)
- [arch::arm::vaddq_f32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_f32.html)
- [arch::arm::vaddq_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_s16.html)
- [arch::arm::vaddq_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_s32.html)
- [arch::arm::vaddq_s64](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_s64.html)
- [arch::arm::vaddq_s8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_s8.html)
- [arch::arm::vaddq_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_u16.html)
- [arch::arm::vaddq_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_u32.html)
- [arch::arm::vaddq_u64](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_u64.html)
- [arch::arm::vaddq_u8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vaddq_u8.html)
- [arch::arm::vmovl_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovl_s16.html)
- [arch::arm::vmovl_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovl_s32.html)
- [arch::arm::vmovl_s8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovl_s8.html)
- [arch::arm::vmovl_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovl_u16.html)
- [arch::arm::vmovl_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovl_u32.html)
- [arch::arm::vmovl_u8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovl_u8.html)
- [arch::arm::vmovn_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovn_s16.html)
- [arch::arm::vmovn_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovn_s32.html)
- [arch::arm::vmovn_s64](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovn_s64.html)
- [arch::arm::vmovn_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovn_u16.html)
- [arch::arm::vmovn_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovn_u32.html)
- [arch::arm::vmovn_u64](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vmovn_u64.html)
- [arch::arm::vpmax_f32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_f32.html)
- [arch::arm::vpmax_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_s16.html)
- [arch::arm::vpmax_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_s32.html)
- [arch::arm::vpmax_s8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_s8.html)
- [arch::arm::vpmax_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_u16.html)
- [arch::arm::vpmax_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_u32.html)
- [arch::arm::vpmax_u8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmax_u8.html)
- [arch::arm::vpmin_f32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_f32.html)
- [arch::arm::vpmin_s16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_s16.html)
- [arch::arm::vpmin_s32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_s32.html)
- [arch::arm::vpmin_s8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_s8.html)
- [arch::arm::vpmin_u16](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_u16.html)
- [arch::arm::vpmin_u32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_u32.html)
- [arch::arm::vpmin_u8](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vpmin_u8.html)
- [arch::arm::vrsqrte_f32](https://doc.rust-lang.org/nightly/core/arch/arm/fn.vrsqrte_f32.html)
- [arch::mips64::__msa_add_a_b](https://doc.rust-lang.org/nightly/core/arch/mips64/fn.__msa_add_a_b.html)
- [arch::mips::__msa_add_a_b](https://doc.rust-lang.org/nightly/core/arch/mips/fn.__msa_add_a_b.html)
- [arch::x86::_MM_GET_EXCEPTION_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_GET_EXCEPTION_MASK.html)
- [arch::x86::_MM_GET_EXCEPTION_STATE](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_GET_EXCEPTION_STATE.html)
- [arch::x86::_MM_GET_FLUSH_ZERO_MODE](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_GET_FLUSH_ZERO_MODE.html)
- [arch::x86::_MM_GET_ROUNDING_MODE](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_GET_ROUNDING_MODE.html)
- [arch::x86::_MM_SET_EXCEPTION_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_SET_EXCEPTION_MASK.html)
- [arch::x86::_MM_SET_EXCEPTION_STATE](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_SET_EXCEPTION_STATE.html)
- [arch::x86::_MM_SET_FLUSH_ZERO_MODE](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_SET_FLUSH_ZERO_MODE.html)
- [arch::x86::_MM_SET_ROUNDING_MODE](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_SET_ROUNDING_MODE.html)
- [arch::x86::_MM_TRANSPOSE4_PS](https://doc.rust-lang.org/nightly/core/arch/x86/fn._MM_TRANSPOSE4_PS.html)
- [arch::x86::__cpuid](https://doc.rust-lang.org/nightly/core/arch/x86/fn.__cpuid.html)
- [arch::x86::__cpuid_count](https://doc.rust-lang.org/nightly/core/arch/x86/fn.__cpuid_count.html)
- [arch::x86::__get_cpuid_max](https://doc.rust-lang.org/nightly/core/arch/x86/fn.__get_cpuid_max.html)
- [arch::x86::__rdtscp](https://doc.rust-lang.org/nightly/core/arch/x86/fn.__rdtscp.html)
- [arch::x86::__readeflags](https://doc.rust-lang.org/nightly/core/arch/x86/fn.__readeflags.html)
- [arch::x86::__writeeflags](https://doc.rust-lang.org/nightly/core/arch/x86/fn.__writeeflags.html)
- [arch::x86::_andn_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._andn_u32.html)
- [arch::x86::_bextr2_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._bextr2_u32.html)
- [arch::x86::_bextr_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._bextr_u32.html)
- [arch::x86::_blcfill_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcfill_u32.html)
- [arch::x86::_blcfill_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcfill_u64.html)
- [arch::x86::_blci_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blci_u32.html)
- [arch::x86::_blci_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blci_u64.html)
- [arch::x86::_blcic_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcic_u32.html)
- [arch::x86::_blcic_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcic_u64.html)
- [arch::x86::_blcmsk_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcmsk_u32.html)
- [arch::x86::_blcmsk_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcmsk_u64.html)
- [arch::x86::_blcs_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcs_u32.html)
- [arch::x86::_blcs_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blcs_u64.html)
- [arch::x86::_blsfill_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsfill_u32.html)
- [arch::x86::_blsfill_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsfill_u64.html)
- [arch::x86::_blsi_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsi_u32.html)
- [arch::x86::_blsic_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsic_u32.html)
- [arch::x86::_blsic_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsic_u64.html)
- [arch::x86::_blsmsk_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsmsk_u32.html)
- [arch::x86::_blsr_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._blsr_u32.html)
- [arch::x86::_bswap](https://doc.rust-lang.org/nightly/core/arch/x86/fn._bswap.html)
- [arch::x86::_bzhi_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._bzhi_u32.html)
- [arch::x86::_fxrstor](https://doc.rust-lang.org/nightly/core/arch/x86/fn._fxrstor.html)
- [arch::x86::_fxsave](https://doc.rust-lang.org/nightly/core/arch/x86/fn._fxsave.html)
- [arch::x86::_lzcnt_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._lzcnt_u32.html)
- [arch::x86::_m_maskmovq](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_maskmovq.html)
- [arch::x86::_m_paddb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddb.html)
- [arch::x86::_m_paddd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddd.html)
- [arch::x86::_m_paddsb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddsb.html)
- [arch::x86::_m_paddsw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddsw.html)
- [arch::x86::_m_paddusb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddusb.html)
- [arch::x86::_m_paddusw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddusw.html)
- [arch::x86::_m_paddw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_paddw.html)
- [arch::x86::_m_pavgb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pavgb.html)
- [arch::x86::_m_pavgw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pavgw.html)
- [arch::x86::_m_pextrw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pextrw.html)
- [arch::x86::_m_pinsrw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pinsrw.html)
- [arch::x86::_m_pmaxsw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pmaxsw.html)
- [arch::x86::_m_pmaxub](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pmaxub.html)
- [arch::x86::_m_pminsw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pminsw.html)
- [arch::x86::_m_pminub](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pminub.html)
- [arch::x86::_m_pmovmskb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pmovmskb.html)
- [arch::x86::_m_pmulhuw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pmulhuw.html)
- [arch::x86::_m_psadbw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psadbw.html)
- [arch::x86::_m_pshufw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_pshufw.html)
- [arch::x86::_m_psubb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubb.html)
- [arch::x86::_m_psubd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubd.html)
- [arch::x86::_m_psubsb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubsb.html)
- [arch::x86::_m_psubsw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubsw.html)
- [arch::x86::_m_psubusb](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubusb.html)
- [arch::x86::_m_psubusw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubusw.html)
- [arch::x86::_m_psubw](https://doc.rust-lang.org/nightly/core/arch/x86/fn._m_psubw.html)
- [arch::x86::_mm256_abs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_abs_epi16.html)
- [arch::x86::_mm256_abs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_abs_epi32.html)
- [arch::x86::_mm256_abs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_abs_epi8.html)
- [arch::x86::_mm256_add_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_add_epi16.html)
- [arch::x86::_mm256_add_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_add_epi32.html)
- [arch::x86::_mm256_add_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_add_epi64.html)
- [arch::x86::_mm256_add_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_add_epi8.html)
- [arch::x86::_mm256_add_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_add_pd.html)
- [arch::x86::_mm256_add_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_add_ps.html)
- [arch::x86::_mm256_adds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_adds_epi16.html)
- [arch::x86::_mm256_adds_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_adds_epi8.html)
- [arch::x86::_mm256_adds_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_adds_epu16.html)
- [arch::x86::_mm256_adds_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_adds_epu8.html)
- [arch::x86::_mm256_addsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_addsub_pd.html)
- [arch::x86::_mm256_addsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_addsub_ps.html)
- [arch::x86::_mm256_alignr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_alignr_epi8.html)
- [arch::x86::_mm256_and_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_and_pd.html)
- [arch::x86::_mm256_and_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_and_ps.html)
- [arch::x86::_mm256_and_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_and_si256.html)
- [arch::x86::_mm256_andnot_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_andnot_pd.html)
- [arch::x86::_mm256_andnot_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_andnot_ps.html)
- [arch::x86::_mm256_andnot_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_andnot_si256.html)
- [arch::x86::_mm256_avg_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_avg_epu16.html)
- [arch::x86::_mm256_avg_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_avg_epu8.html)
- [arch::x86::_mm256_blend_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blend_epi16.html)
- [arch::x86::_mm256_blend_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blend_epi32.html)
- [arch::x86::_mm256_blend_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blend_pd.html)
- [arch::x86::_mm256_blend_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blend_ps.html)
- [arch::x86::_mm256_blendv_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blendv_epi8.html)
- [arch::x86::_mm256_blendv_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blendv_pd.html)
- [arch::x86::_mm256_blendv_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_blendv_ps.html)
- [arch::x86::_mm256_broadcast_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcast_pd.html)
- [arch::x86::_mm256_broadcast_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcast_ps.html)
- [arch::x86::_mm256_broadcast_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcast_sd.html)
- [arch::x86::_mm256_broadcast_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcast_ss.html)
- [arch::x86::_mm256_broadcastb_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastb_epi8.html)
- [arch::x86::_mm256_broadcastd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastd_epi32.html)
- [arch::x86::_mm256_broadcastq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastq_epi64.html)
- [arch::x86::_mm256_broadcastsd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastsd_pd.html)
- [arch::x86::_mm256_broadcastsi128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastsi128_si256.html)
- [arch::x86::_mm256_broadcastss_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastss_ps.html)
- [arch::x86::_mm256_broadcastw_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_broadcastw_epi16.html)
- [arch::x86::_mm256_bslli_epi128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_bslli_epi128.html)
- [arch::x86::_mm256_bsrli_epi128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_bsrli_epi128.html)
- [arch::x86::_mm256_castpd128_pd256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castpd128_pd256.html)
- [arch::x86::_mm256_castpd256_pd128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castpd256_pd128.html)
- [arch::x86::_mm256_castpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castpd_ps.html)
- [arch::x86::_mm256_castpd_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castpd_si256.html)
- [arch::x86::_mm256_castps128_ps256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castps128_ps256.html)
- [arch::x86::_mm256_castps256_ps128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castps256_ps128.html)
- [arch::x86::_mm256_castps_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castps_pd.html)
- [arch::x86::_mm256_castps_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castps_si256.html)
- [arch::x86::_mm256_castsi128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castsi128_si256.html)
- [arch::x86::_mm256_castsi256_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castsi256_pd.html)
- [arch::x86::_mm256_castsi256_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castsi256_ps.html)
- [arch::x86::_mm256_castsi256_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_castsi256_si128.html)
- [arch::x86::_mm256_ceil_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_ceil_pd.html)
- [arch::x86::_mm256_ceil_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_ceil_ps.html)
- [arch::x86::_mm256_cmp_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmp_pd.html)
- [arch::x86::_mm256_cmp_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmp_ps.html)
- [arch::x86::_mm256_cmpeq_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpeq_epi16.html)
- [arch::x86::_mm256_cmpeq_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpeq_epi32.html)
- [arch::x86::_mm256_cmpeq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpeq_epi64.html)
- [arch::x86::_mm256_cmpeq_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpeq_epi8.html)
- [arch::x86::_mm256_cmpgt_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpgt_epi16.html)
- [arch::x86::_mm256_cmpgt_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpgt_epi32.html)
- [arch::x86::_mm256_cmpgt_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpgt_epi64.html)
- [arch::x86::_mm256_cmpgt_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cmpgt_epi8.html)
- [arch::x86::_mm256_cvtepi16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi16_epi32.html)
- [arch::x86::_mm256_cvtepi16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi16_epi64.html)
- [arch::x86::_mm256_cvtepi32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi32_epi64.html)
- [arch::x86::_mm256_cvtepi32_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi32_pd.html)
- [arch::x86::_mm256_cvtepi32_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi32_ps.html)
- [arch::x86::_mm256_cvtepi8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi8_epi16.html)
- [arch::x86::_mm256_cvtepi8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi8_epi32.html)
- [arch::x86::_mm256_cvtepi8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepi8_epi64.html)
- [arch::x86::_mm256_cvtepu16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepu16_epi32.html)
- [arch::x86::_mm256_cvtepu16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepu16_epi64.html)
- [arch::x86::_mm256_cvtepu32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepu32_epi64.html)
- [arch::x86::_mm256_cvtepu8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepu8_epi16.html)
- [arch::x86::_mm256_cvtepu8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepu8_epi32.html)
- [arch::x86::_mm256_cvtepu8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtepu8_epi64.html)
- [arch::x86::_mm256_cvtpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtpd_epi32.html)
- [arch::x86::_mm256_cvtpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtpd_ps.html)
- [arch::x86::_mm256_cvtps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtps_epi32.html)
- [arch::x86::_mm256_cvtps_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtps_pd.html)
- [arch::x86::_mm256_cvtsd_f64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtsd_f64.html)
- [arch::x86::_mm256_cvtsi256_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtsi256_si32.html)
- [arch::x86::_mm256_cvtss_f32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvtss_f32.html)
- [arch::x86::_mm256_cvttpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvttpd_epi32.html)
- [arch::x86::_mm256_cvttps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_cvttps_epi32.html)
- [arch::x86::_mm256_div_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_div_pd.html)
- [arch::x86::_mm256_div_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_div_ps.html)
- [arch::x86::_mm256_dp_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_dp_ps.html)
- [arch::x86::_mm256_extract_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extract_epi16.html)
- [arch::x86::_mm256_extract_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extract_epi32.html)
- [arch::x86::_mm256_extract_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extract_epi8.html)
- [arch::x86::_mm256_extractf128_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extractf128_pd.html)
- [arch::x86::_mm256_extractf128_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extractf128_ps.html)
- [arch::x86::_mm256_extractf128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extractf128_si256.html)
- [arch::x86::_mm256_extracti128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_extracti128_si256.html)
- [arch::x86::_mm256_floor_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_floor_pd.html)
- [arch::x86::_mm256_floor_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_floor_ps.html)
- [arch::x86::_mm256_fmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmadd_pd.html)
- [arch::x86::_mm256_fmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmadd_ps.html)
- [arch::x86::_mm256_fmaddsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmaddsub_pd.html)
- [arch::x86::_mm256_fmaddsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmaddsub_ps.html)
- [arch::x86::_mm256_fmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmsub_pd.html)
- [arch::x86::_mm256_fmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmsub_ps.html)
- [arch::x86::_mm256_fmsubadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmsubadd_pd.html)
- [arch::x86::_mm256_fmsubadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fmsubadd_ps.html)
- [arch::x86::_mm256_fnmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fnmadd_pd.html)
- [arch::x86::_mm256_fnmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fnmadd_ps.html)
- [arch::x86::_mm256_fnmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fnmsub_pd.html)
- [arch::x86::_mm256_fnmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_fnmsub_ps.html)
- [arch::x86::_mm256_hadd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hadd_epi16.html)
- [arch::x86::_mm256_hadd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hadd_epi32.html)
- [arch::x86::_mm256_hadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hadd_pd.html)
- [arch::x86::_mm256_hadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hadd_ps.html)
- [arch::x86::_mm256_hadds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hadds_epi16.html)
- [arch::x86::_mm256_hsub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hsub_epi16.html)
- [arch::x86::_mm256_hsub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hsub_epi32.html)
- [arch::x86::_mm256_hsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hsub_pd.html)
- [arch::x86::_mm256_hsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hsub_ps.html)
- [arch::x86::_mm256_hsubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_hsubs_epi16.html)
- [arch::x86::_mm256_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i32gather_epi32.html)
- [arch::x86::_mm256_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i32gather_epi64.html)
- [arch::x86::_mm256_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i32gather_pd.html)
- [arch::x86::_mm256_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i32gather_ps.html)
- [arch::x86::_mm256_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i64gather_epi32.html)
- [arch::x86::_mm256_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i64gather_epi64.html)
- [arch::x86::_mm256_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i64gather_pd.html)
- [arch::x86::_mm256_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_i64gather_ps.html)
- [arch::x86::_mm256_insert_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_insert_epi16.html)
- [arch::x86::_mm256_insert_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_insert_epi32.html)
- [arch::x86::_mm256_insert_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_insert_epi8.html)
- [arch::x86::_mm256_insertf128_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_insertf128_pd.html)
- [arch::x86::_mm256_insertf128_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_insertf128_ps.html)
- [arch::x86::_mm256_insertf128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_insertf128_si256.html)
- [arch::x86::_mm256_inserti128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_inserti128_si256.html)
- [arch::x86::_mm256_lddqu_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_lddqu_si256.html)
- [arch::x86::_mm256_load_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_load_pd.html)
- [arch::x86::_mm256_load_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_load_ps.html)
- [arch::x86::_mm256_load_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_load_si256.html)
- [arch::x86::_mm256_loadu2_m128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_loadu2_m128.html)
- [arch::x86::_mm256_loadu2_m128d](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_loadu2_m128d.html)
- [arch::x86::_mm256_loadu2_m128i](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_loadu2_m128i.html)
- [arch::x86::_mm256_loadu_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_loadu_pd.html)
- [arch::x86::_mm256_loadu_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_loadu_ps.html)
- [arch::x86::_mm256_loadu_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_loadu_si256.html)
- [arch::x86::_mm256_madd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_madd_epi16.html)
- [arch::x86::_mm256_maddubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maddubs_epi16.html)
- [arch::x86::_mm256_mask_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i32gather_epi32.html)
- [arch::x86::_mm256_mask_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i32gather_epi64.html)
- [arch::x86::_mm256_mask_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i32gather_pd.html)
- [arch::x86::_mm256_mask_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i32gather_ps.html)
- [arch::x86::_mm256_mask_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i64gather_epi32.html)
- [arch::x86::_mm256_mask_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i64gather_epi64.html)
- [arch::x86::_mm256_mask_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i64gather_pd.html)
- [arch::x86::_mm256_mask_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mask_i64gather_ps.html)
- [arch::x86::_mm256_maskload_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskload_epi32.html)
- [arch::x86::_mm256_maskload_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskload_epi64.html)
- [arch::x86::_mm256_maskload_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskload_pd.html)
- [arch::x86::_mm256_maskload_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskload_ps.html)
- [arch::x86::_mm256_maskstore_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskstore_epi32.html)
- [arch::x86::_mm256_maskstore_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskstore_epi64.html)
- [arch::x86::_mm256_maskstore_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskstore_pd.html)
- [arch::x86::_mm256_maskstore_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_maskstore_ps.html)
- [arch::x86::_mm256_max_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_epi16.html)
- [arch::x86::_mm256_max_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_epi32.html)
- [arch::x86::_mm256_max_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_epi8.html)
- [arch::x86::_mm256_max_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_epu16.html)
- [arch::x86::_mm256_max_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_epu32.html)
- [arch::x86::_mm256_max_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_epu8.html)
- [arch::x86::_mm256_max_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_pd.html)
- [arch::x86::_mm256_max_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_max_ps.html)
- [arch::x86::_mm256_min_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_epi16.html)
- [arch::x86::_mm256_min_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_epi32.html)
- [arch::x86::_mm256_min_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_epi8.html)
- [arch::x86::_mm256_min_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_epu16.html)
- [arch::x86::_mm256_min_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_epu32.html)
- [arch::x86::_mm256_min_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_epu8.html)
- [arch::x86::_mm256_min_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_pd.html)
- [arch::x86::_mm256_min_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_min_ps.html)
- [arch::x86::_mm256_movedup_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_movedup_pd.html)
- [arch::x86::_mm256_movehdup_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_movehdup_ps.html)
- [arch::x86::_mm256_moveldup_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_moveldup_ps.html)
- [arch::x86::_mm256_movemask_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_movemask_epi8.html)
- [arch::x86::_mm256_movemask_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_movemask_pd.html)
- [arch::x86::_mm256_movemask_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_movemask_ps.html)
- [arch::x86::_mm256_mpsadbw_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mpsadbw_epu8.html)
- [arch::x86::_mm256_mul_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mul_epi32.html)
- [arch::x86::_mm256_mul_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mul_epu32.html)
- [arch::x86::_mm256_mul_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mul_pd.html)
- [arch::x86::_mm256_mul_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mul_ps.html)
- [arch::x86::_mm256_mulhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mulhi_epi16.html)
- [arch::x86::_mm256_mulhi_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mulhi_epu16.html)
- [arch::x86::_mm256_mulhrs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mulhrs_epi16.html)
- [arch::x86::_mm256_mullo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mullo_epi16.html)
- [arch::x86::_mm256_mullo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_mullo_epi32.html)
- [arch::x86::_mm256_or_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_or_pd.html)
- [arch::x86::_mm256_or_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_or_ps.html)
- [arch::x86::_mm256_or_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_or_si256.html)
- [arch::x86::_mm256_packs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_packs_epi16.html)
- [arch::x86::_mm256_packs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_packs_epi32.html)
- [arch::x86::_mm256_packus_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_packus_epi16.html)
- [arch::x86::_mm256_packus_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_packus_epi32.html)
- [arch::x86::_mm256_permute2f128_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute2f128_pd.html)
- [arch::x86::_mm256_permute2f128_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute2f128_ps.html)
- [arch::x86::_mm256_permute2f128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute2f128_si256.html)
- [arch::x86::_mm256_permute2x128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute2x128_si256.html)
- [arch::x86::_mm256_permute4x64_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute4x64_epi64.html)
- [arch::x86::_mm256_permute4x64_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute4x64_pd.html)
- [arch::x86::_mm256_permute_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute_pd.html)
- [arch::x86::_mm256_permute_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permute_ps.html)
- [arch::x86::_mm256_permutevar8x32_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permutevar8x32_epi32.html)
- [arch::x86::_mm256_permutevar8x32_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permutevar8x32_ps.html)
- [arch::x86::_mm256_permutevar_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permutevar_pd.html)
- [arch::x86::_mm256_permutevar_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_permutevar_ps.html)
- [arch::x86::_mm256_rcp_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_rcp_ps.html)
- [arch::x86::_mm256_round_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_round_pd.html)
- [arch::x86::_mm256_round_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_round_ps.html)
- [arch::x86::_mm256_rsqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_rsqrt_ps.html)
- [arch::x86::_mm256_sad_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sad_epu8.html)
- [arch::x86::_mm256_set1_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set1_epi16.html)
- [arch::x86::_mm256_set1_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set1_epi32.html)
- [arch::x86::_mm256_set1_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set1_epi64x.html)
- [arch::x86::_mm256_set1_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set1_epi8.html)
- [arch::x86::_mm256_set1_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set1_pd.html)
- [arch::x86::_mm256_set1_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set1_ps.html)
- [arch::x86::_mm256_set_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_epi16.html)
- [arch::x86::_mm256_set_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_epi32.html)
- [arch::x86::_mm256_set_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_epi64x.html)
- [arch::x86::_mm256_set_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_epi8.html)
- [arch::x86::_mm256_set_m128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_m128.html)
- [arch::x86::_mm256_set_m128d](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_m128d.html)
- [arch::x86::_mm256_set_m128i](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_m128i.html)
- [arch::x86::_mm256_set_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_pd.html)
- [arch::x86::_mm256_set_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_set_ps.html)
- [arch::x86::_mm256_setr_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_epi16.html)
- [arch::x86::_mm256_setr_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_epi32.html)
- [arch::x86::_mm256_setr_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_epi64x.html)
- [arch::x86::_mm256_setr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_epi8.html)
- [arch::x86::_mm256_setr_m128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_m128.html)
- [arch::x86::_mm256_setr_m128d](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_m128d.html)
- [arch::x86::_mm256_setr_m128i](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_m128i.html)
- [arch::x86::_mm256_setr_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_pd.html)
- [arch::x86::_mm256_setr_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setr_ps.html)
- [arch::x86::_mm256_setzero_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setzero_pd.html)
- [arch::x86::_mm256_setzero_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setzero_ps.html)
- [arch::x86::_mm256_setzero_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_setzero_si256.html)
- [arch::x86::_mm256_shuffle_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_shuffle_epi32.html)
- [arch::x86::_mm256_shuffle_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_shuffle_epi8.html)
- [arch::x86::_mm256_shuffle_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_shuffle_pd.html)
- [arch::x86::_mm256_shuffle_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_shuffle_ps.html)
- [arch::x86::_mm256_shufflehi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_shufflehi_epi16.html)
- [arch::x86::_mm256_shufflelo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_shufflelo_epi16.html)
- [arch::x86::_mm256_sign_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sign_epi16.html)
- [arch::x86::_mm256_sign_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sign_epi32.html)
- [arch::x86::_mm256_sign_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sign_epi8.html)
- [arch::x86::_mm256_sll_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sll_epi16.html)
- [arch::x86::_mm256_sll_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sll_epi32.html)
- [arch::x86::_mm256_sll_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sll_epi64.html)
- [arch::x86::_mm256_slli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_slli_epi16.html)
- [arch::x86::_mm256_slli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_slli_epi32.html)
- [arch::x86::_mm256_slli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_slli_epi64.html)
- [arch::x86::_mm256_slli_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_slli_si256.html)
- [arch::x86::_mm256_sllv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sllv_epi32.html)
- [arch::x86::_mm256_sllv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sllv_epi64.html)
- [arch::x86::_mm256_sqrt_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sqrt_pd.html)
- [arch::x86::_mm256_sqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sqrt_ps.html)
- [arch::x86::_mm256_sra_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sra_epi16.html)
- [arch::x86::_mm256_sra_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sra_epi32.html)
- [arch::x86::_mm256_srai_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srai_epi16.html)
- [arch::x86::_mm256_srai_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srai_epi32.html)
- [arch::x86::_mm256_srav_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srav_epi32.html)
- [arch::x86::_mm256_srl_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srl_epi16.html)
- [arch::x86::_mm256_srl_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srl_epi32.html)
- [arch::x86::_mm256_srl_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srl_epi64.html)
- [arch::x86::_mm256_srli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srli_epi16.html)
- [arch::x86::_mm256_srli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srli_epi32.html)
- [arch::x86::_mm256_srli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srli_epi64.html)
- [arch::x86::_mm256_srli_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srli_si256.html)
- [arch::x86::_mm256_srlv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srlv_epi32.html)
- [arch::x86::_mm256_srlv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_srlv_epi64.html)
- [arch::x86::_mm256_store_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_store_pd.html)
- [arch::x86::_mm256_store_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_store_ps.html)
- [arch::x86::_mm256_store_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_store_si256.html)
- [arch::x86::_mm256_storeu2_m128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_storeu2_m128.html)
- [arch::x86::_mm256_storeu2_m128d](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_storeu2_m128d.html)
- [arch::x86::_mm256_storeu2_m128i](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_storeu2_m128i.html)
- [arch::x86::_mm256_storeu_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_storeu_pd.html)
- [arch::x86::_mm256_storeu_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_storeu_ps.html)
- [arch::x86::_mm256_storeu_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_storeu_si256.html)
- [arch::x86::_mm256_stream_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_stream_pd.html)
- [arch::x86::_mm256_stream_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_stream_ps.html)
- [arch::x86::_mm256_stream_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_stream_si256.html)
- [arch::x86::_mm256_sub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sub_epi16.html)
- [arch::x86::_mm256_sub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sub_epi32.html)
- [arch::x86::_mm256_sub_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sub_epi64.html)
- [arch::x86::_mm256_sub_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sub_epi8.html)
- [arch::x86::_mm256_sub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sub_pd.html)
- [arch::x86::_mm256_sub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_sub_ps.html)
- [arch::x86::_mm256_subs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_subs_epi16.html)
- [arch::x86::_mm256_subs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_subs_epi8.html)
- [arch::x86::_mm256_subs_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_subs_epu16.html)
- [arch::x86::_mm256_subs_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_subs_epu8.html)
- [arch::x86::_mm256_testc_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testc_pd.html)
- [arch::x86::_mm256_testc_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testc_ps.html)
- [arch::x86::_mm256_testc_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testc_si256.html)
- [arch::x86::_mm256_testnzc_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testnzc_pd.html)
- [arch::x86::_mm256_testnzc_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testnzc_ps.html)
- [arch::x86::_mm256_testnzc_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testnzc_si256.html)
- [arch::x86::_mm256_testz_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testz_pd.html)
- [arch::x86::_mm256_testz_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testz_ps.html)
- [arch::x86::_mm256_testz_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_testz_si256.html)
- [arch::x86::_mm256_undefined_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_undefined_pd.html)
- [arch::x86::_mm256_undefined_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_undefined_ps.html)
- [arch::x86::_mm256_undefined_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_undefined_si256.html)
- [arch::x86::_mm256_unpackhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpackhi_epi16.html)
- [arch::x86::_mm256_unpackhi_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpackhi_epi32.html)
- [arch::x86::_mm256_unpackhi_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpackhi_epi64.html)
- [arch::x86::_mm256_unpackhi_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpackhi_epi8.html)
- [arch::x86::_mm256_unpackhi_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpackhi_pd.html)
- [arch::x86::_mm256_unpackhi_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpackhi_ps.html)
- [arch::x86::_mm256_unpacklo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpacklo_epi16.html)
- [arch::x86::_mm256_unpacklo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpacklo_epi32.html)
- [arch::x86::_mm256_unpacklo_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpacklo_epi64.html)
- [arch::x86::_mm256_unpacklo_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpacklo_epi8.html)
- [arch::x86::_mm256_unpacklo_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpacklo_pd.html)
- [arch::x86::_mm256_unpacklo_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_unpacklo_ps.html)
- [arch::x86::_mm256_xor_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_xor_pd.html)
- [arch::x86::_mm256_xor_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_xor_ps.html)
- [arch::x86::_mm256_xor_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_xor_si256.html)
- [arch::x86::_mm256_zeroall](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_zeroall.html)
- [arch::x86::_mm256_zeroupper](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_zeroupper.html)
- [arch::x86::_mm256_zextpd128_pd256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_zextpd128_pd256.html)
- [arch::x86::_mm256_zextps128_ps256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_zextps128_ps256.html)
- [arch::x86::_mm256_zextsi128_si256](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm256_zextsi128_si256.html)
- [arch::x86::_mm_abs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_abs_epi16.html)
- [arch::x86::_mm_abs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_abs_epi32.html)
- [arch::x86::_mm_abs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_abs_epi8.html)
- [arch::x86::_mm_abs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_abs_pi16.html)
- [arch::x86::_mm_abs_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_abs_pi32.html)
- [arch::x86::_mm_abs_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_abs_pi8.html)
- [arch::x86::_mm_add_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_epi16.html)
- [arch::x86::_mm_add_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_epi32.html)
- [arch::x86::_mm_add_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_epi64.html)
- [arch::x86::_mm_add_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_epi8.html)
- [arch::x86::_mm_add_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_pd.html)
- [arch::x86::_mm_add_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_pi16.html)
- [arch::x86::_mm_add_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_pi32.html)
- [arch::x86::_mm_add_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_pi8.html)
- [arch::x86::_mm_add_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_ps.html)
- [arch::x86::_mm_add_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_sd.html)
- [arch::x86::_mm_add_si64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_si64.html)
- [arch::x86::_mm_add_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_add_ss.html)
- [arch::x86::_mm_adds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_epi16.html)
- [arch::x86::_mm_adds_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_epi8.html)
- [arch::x86::_mm_adds_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_epu16.html)
- [arch::x86::_mm_adds_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_epu8.html)
- [arch::x86::_mm_adds_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_pi16.html)
- [arch::x86::_mm_adds_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_pi8.html)
- [arch::x86::_mm_adds_pu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_pu16.html)
- [arch::x86::_mm_adds_pu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_adds_pu8.html)
- [arch::x86::_mm_addsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_addsub_pd.html)
- [arch::x86::_mm_addsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_addsub_ps.html)
- [arch::x86::_mm_aesdec_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_aesdec_si128.html)
- [arch::x86::_mm_aesdeclast_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_aesdeclast_si128.html)
- [arch::x86::_mm_aesenc_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_aesenc_si128.html)
- [arch::x86::_mm_aesenclast_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_aesenclast_si128.html)
- [arch::x86::_mm_aesimc_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_aesimc_si128.html)
- [arch::x86::_mm_aeskeygenassist_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_aeskeygenassist_si128.html)
- [arch::x86::_mm_alignr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_alignr_epi8.html)
- [arch::x86::_mm_alignr_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_alignr_pi8.html)
- [arch::x86::_mm_and_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_and_pd.html)
- [arch::x86::_mm_and_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_and_ps.html)
- [arch::x86::_mm_and_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_and_si128.html)
- [arch::x86::_mm_andnot_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_andnot_pd.html)
- [arch::x86::_mm_andnot_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_andnot_ps.html)
- [arch::x86::_mm_andnot_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_andnot_si128.html)
- [arch::x86::_mm_avg_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_avg_epu16.html)
- [arch::x86::_mm_avg_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_avg_epu8.html)
- [arch::x86::_mm_avg_pu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_avg_pu16.html)
- [arch::x86::_mm_avg_pu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_avg_pu8.html)
- [arch::x86::_mm_blend_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blend_epi16.html)
- [arch::x86::_mm_blend_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blend_epi32.html)
- [arch::x86::_mm_blend_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blend_pd.html)
- [arch::x86::_mm_blend_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blend_ps.html)
- [arch::x86::_mm_blendv_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blendv_epi8.html)
- [arch::x86::_mm_blendv_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blendv_pd.html)
- [arch::x86::_mm_blendv_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_blendv_ps.html)
- [arch::x86::_mm_broadcast_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcast_ss.html)
- [arch::x86::_mm_broadcastb_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcastb_epi8.html)
- [arch::x86::_mm_broadcastd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcastd_epi32.html)
- [arch::x86::_mm_broadcastq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcastq_epi64.html)
- [arch::x86::_mm_broadcastsd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcastsd_pd.html)
- [arch::x86::_mm_broadcastss_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcastss_ps.html)
- [arch::x86::_mm_broadcastw_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_broadcastw_epi16.html)
- [arch::x86::_mm_bslli_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_bslli_si128.html)
- [arch::x86::_mm_bsrli_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_bsrli_si128.html)
- [arch::x86::_mm_castpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_castpd_ps.html)
- [arch::x86::_mm_castpd_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_castpd_si128.html)
- [arch::x86::_mm_castps_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_castps_pd.html)
- [arch::x86::_mm_castps_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_castps_si128.html)
- [arch::x86::_mm_castsi128_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_castsi128_pd.html)
- [arch::x86::_mm_castsi128_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_castsi128_ps.html)
- [arch::x86::_mm_ceil_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ceil_pd.html)
- [arch::x86::_mm_ceil_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ceil_ps.html)
- [arch::x86::_mm_ceil_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ceil_sd.html)
- [arch::x86::_mm_ceil_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ceil_ss.html)
- [arch::x86::_mm_clflush](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_clflush.html)
- [arch::x86::_mm_clmulepi64_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_clmulepi64_si128.html)
- [arch::x86::_mm_cmp_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmp_pd.html)
- [arch::x86::_mm_cmp_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmp_ps.html)
- [arch::x86::_mm_cmp_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmp_sd.html)
- [arch::x86::_mm_cmp_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmp_ss.html)
- [arch::x86::_mm_cmpeq_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_epi16.html)
- [arch::x86::_mm_cmpeq_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_epi32.html)
- [arch::x86::_mm_cmpeq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_epi64.html)
- [arch::x86::_mm_cmpeq_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_epi8.html)
- [arch::x86::_mm_cmpeq_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_pd.html)
- [arch::x86::_mm_cmpeq_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_ps.html)
- [arch::x86::_mm_cmpeq_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_sd.html)
- [arch::x86::_mm_cmpeq_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpeq_ss.html)
- [arch::x86::_mm_cmpestra](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestra.html)
- [arch::x86::_mm_cmpestrc](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestrc.html)
- [arch::x86::_mm_cmpestri](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestri.html)
- [arch::x86::_mm_cmpestrm](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestrm.html)
- [arch::x86::_mm_cmpestro](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestro.html)
- [arch::x86::_mm_cmpestrs](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestrs.html)
- [arch::x86::_mm_cmpestrz](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpestrz.html)
- [arch::x86::_mm_cmpge_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpge_pd.html)
- [arch::x86::_mm_cmpge_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpge_ps.html)
- [arch::x86::_mm_cmpge_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpge_sd.html)
- [arch::x86::_mm_cmpge_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpge_ss.html)
- [arch::x86::_mm_cmpgt_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_epi16.html)
- [arch::x86::_mm_cmpgt_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_epi32.html)
- [arch::x86::_mm_cmpgt_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_epi64.html)
- [arch::x86::_mm_cmpgt_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_epi8.html)
- [arch::x86::_mm_cmpgt_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_pd.html)
- [arch::x86::_mm_cmpgt_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_pi16.html)
- [arch::x86::_mm_cmpgt_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_pi32.html)
- [arch::x86::_mm_cmpgt_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_pi8.html)
- [arch::x86::_mm_cmpgt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_ps.html)
- [arch::x86::_mm_cmpgt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_sd.html)
- [arch::x86::_mm_cmpgt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpgt_ss.html)
- [arch::x86::_mm_cmpistra](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistra.html)
- [arch::x86::_mm_cmpistrc](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistrc.html)
- [arch::x86::_mm_cmpistri](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistri.html)
- [arch::x86::_mm_cmpistrm](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistrm.html)
- [arch::x86::_mm_cmpistro](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistro.html)
- [arch::x86::_mm_cmpistrs](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistrs.html)
- [arch::x86::_mm_cmpistrz](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpistrz.html)
- [arch::x86::_mm_cmple_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmple_pd.html)
- [arch::x86::_mm_cmple_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmple_ps.html)
- [arch::x86::_mm_cmple_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmple_sd.html)
- [arch::x86::_mm_cmple_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmple_ss.html)
- [arch::x86::_mm_cmplt_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_epi16.html)
- [arch::x86::_mm_cmplt_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_epi32.html)
- [arch::x86::_mm_cmplt_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_epi8.html)
- [arch::x86::_mm_cmplt_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_pd.html)
- [arch::x86::_mm_cmplt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_ps.html)
- [arch::x86::_mm_cmplt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_sd.html)
- [arch::x86::_mm_cmplt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmplt_ss.html)
- [arch::x86::_mm_cmpneq_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpneq_pd.html)
- [arch::x86::_mm_cmpneq_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpneq_ps.html)
- [arch::x86::_mm_cmpneq_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpneq_sd.html)
- [arch::x86::_mm_cmpneq_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpneq_ss.html)
- [arch::x86::_mm_cmpnge_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnge_pd.html)
- [arch::x86::_mm_cmpnge_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnge_ps.html)
- [arch::x86::_mm_cmpnge_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnge_sd.html)
- [arch::x86::_mm_cmpnge_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnge_ss.html)
- [arch::x86::_mm_cmpngt_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpngt_pd.html)
- [arch::x86::_mm_cmpngt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpngt_ps.html)
- [arch::x86::_mm_cmpngt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpngt_sd.html)
- [arch::x86::_mm_cmpngt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpngt_ss.html)
- [arch::x86::_mm_cmpnle_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnle_pd.html)
- [arch::x86::_mm_cmpnle_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnle_ps.html)
- [arch::x86::_mm_cmpnle_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnle_sd.html)
- [arch::x86::_mm_cmpnle_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnle_ss.html)
- [arch::x86::_mm_cmpnlt_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnlt_pd.html)
- [arch::x86::_mm_cmpnlt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnlt_ps.html)
- [arch::x86::_mm_cmpnlt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnlt_sd.html)
- [arch::x86::_mm_cmpnlt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpnlt_ss.html)
- [arch::x86::_mm_cmpord_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpord_pd.html)
- [arch::x86::_mm_cmpord_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpord_ps.html)
- [arch::x86::_mm_cmpord_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpord_sd.html)
- [arch::x86::_mm_cmpord_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpord_ss.html)
- [arch::x86::_mm_cmpunord_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpunord_pd.html)
- [arch::x86::_mm_cmpunord_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpunord_ps.html)
- [arch::x86::_mm_cmpunord_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpunord_sd.html)
- [arch::x86::_mm_cmpunord_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cmpunord_ss.html)
- [arch::x86::_mm_comieq_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comieq_sd.html)
- [arch::x86::_mm_comieq_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comieq_ss.html)
- [arch::x86::_mm_comige_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comige_sd.html)
- [arch::x86::_mm_comige_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comige_ss.html)
- [arch::x86::_mm_comigt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comigt_sd.html)
- [arch::x86::_mm_comigt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comigt_ss.html)
- [arch::x86::_mm_comile_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comile_sd.html)
- [arch::x86::_mm_comile_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comile_ss.html)
- [arch::x86::_mm_comilt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comilt_sd.html)
- [arch::x86::_mm_comilt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comilt_ss.html)
- [arch::x86::_mm_comineq_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comineq_sd.html)
- [arch::x86::_mm_comineq_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_comineq_ss.html)
- [arch::x86::_mm_crc32_u16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_crc32_u16.html)
- [arch::x86::_mm_crc32_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_crc32_u32.html)
- [arch::x86::_mm_crc32_u8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_crc32_u8.html)
- [arch::x86::_mm_cvt_pi2ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvt_pi2ps.html)
- [arch::x86::_mm_cvt_ps2pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvt_ps2pi.html)
- [arch::x86::_mm_cvt_si2ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvt_si2ss.html)
- [arch::x86::_mm_cvt_ss2si](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvt_ss2si.html)
- [arch::x86::_mm_cvtepi16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi16_epi32.html)
- [arch::x86::_mm_cvtepi16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi16_epi64.html)
- [arch::x86::_mm_cvtepi32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi32_epi64.html)
- [arch::x86::_mm_cvtepi32_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi32_pd.html)
- [arch::x86::_mm_cvtepi32_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi32_ps.html)
- [arch::x86::_mm_cvtepi8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi8_epi16.html)
- [arch::x86::_mm_cvtepi8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi8_epi32.html)
- [arch::x86::_mm_cvtepi8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepi8_epi64.html)
- [arch::x86::_mm_cvtepu16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepu16_epi32.html)
- [arch::x86::_mm_cvtepu16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepu16_epi64.html)
- [arch::x86::_mm_cvtepu32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepu32_epi64.html)
- [arch::x86::_mm_cvtepu8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepu8_epi16.html)
- [arch::x86::_mm_cvtepu8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepu8_epi32.html)
- [arch::x86::_mm_cvtepu8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtepu8_epi64.html)
- [arch::x86::_mm_cvtpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpd_epi32.html)
- [arch::x86::_mm_cvtpd_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpd_pi32.html)
- [arch::x86::_mm_cvtpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpd_ps.html)
- [arch::x86::_mm_cvtpi16_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpi16_ps.html)
- [arch::x86::_mm_cvtpi32_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpi32_pd.html)
- [arch::x86::_mm_cvtpi32_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpi32_ps.html)
- [arch::x86::_mm_cvtpi32x2_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpi32x2_ps.html)
- [arch::x86::_mm_cvtpi8_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpi8_ps.html)
- [arch::x86::_mm_cvtps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtps_epi32.html)
- [arch::x86::_mm_cvtps_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtps_pd.html)
- [arch::x86::_mm_cvtps_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtps_pi16.html)
- [arch::x86::_mm_cvtps_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtps_pi32.html)
- [arch::x86::_mm_cvtps_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtps_pi8.html)
- [arch::x86::_mm_cvtpu16_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpu16_ps.html)
- [arch::x86::_mm_cvtpu8_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtpu8_ps.html)
- [arch::x86::_mm_cvtsd_f64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsd_f64.html)
- [arch::x86::_mm_cvtsd_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsd_si32.html)
- [arch::x86::_mm_cvtsd_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsd_ss.html)
- [arch::x86::_mm_cvtsi128_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsi128_si32.html)
- [arch::x86::_mm_cvtsi32_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsi32_sd.html)
- [arch::x86::_mm_cvtsi32_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsi32_si128.html)
- [arch::x86::_mm_cvtsi32_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtsi32_ss.html)
- [arch::x86::_mm_cvtss_f32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtss_f32.html)
- [arch::x86::_mm_cvtss_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtss_sd.html)
- [arch::x86::_mm_cvtss_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtss_si32.html)
- [arch::x86::_mm_cvtt_ps2pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtt_ps2pi.html)
- [arch::x86::_mm_cvtt_ss2si](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvtt_ss2si.html)
- [arch::x86::_mm_cvttpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvttpd_epi32.html)
- [arch::x86::_mm_cvttpd_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvttpd_pi32.html)
- [arch::x86::_mm_cvttps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvttps_epi32.html)
- [arch::x86::_mm_cvttps_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvttps_pi32.html)
- [arch::x86::_mm_cvttsd_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvttsd_si32.html)
- [arch::x86::_mm_cvttss_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_cvttss_si32.html)
- [arch::x86::_mm_div_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_div_pd.html)
- [arch::x86::_mm_div_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_div_ps.html)
- [arch::x86::_mm_div_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_div_sd.html)
- [arch::x86::_mm_div_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_div_ss.html)
- [arch::x86::_mm_dp_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_dp_pd.html)
- [arch::x86::_mm_dp_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_dp_ps.html)
- [arch::x86::_mm_extract_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_extract_epi16.html)
- [arch::x86::_mm_extract_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_extract_epi32.html)
- [arch::x86::_mm_extract_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_extract_epi8.html)
- [arch::x86::_mm_extract_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_extract_pi16.html)
- [arch::x86::_mm_extract_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_extract_ps.html)
- [arch::x86::_mm_extract_si64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_extract_si64.html)
- [arch::x86::_mm_floor_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_floor_pd.html)
- [arch::x86::_mm_floor_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_floor_ps.html)
- [arch::x86::_mm_floor_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_floor_sd.html)
- [arch::x86::_mm_floor_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_floor_ss.html)
- [arch::x86::_mm_fmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmadd_pd.html)
- [arch::x86::_mm_fmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmadd_ps.html)
- [arch::x86::_mm_fmadd_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmadd_sd.html)
- [arch::x86::_mm_fmadd_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmadd_ss.html)
- [arch::x86::_mm_fmaddsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmaddsub_pd.html)
- [arch::x86::_mm_fmaddsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmaddsub_ps.html)
- [arch::x86::_mm_fmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmsub_pd.html)
- [arch::x86::_mm_fmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmsub_ps.html)
- [arch::x86::_mm_fmsub_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmsub_sd.html)
- [arch::x86::_mm_fmsub_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmsub_ss.html)
- [arch::x86::_mm_fmsubadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmsubadd_pd.html)
- [arch::x86::_mm_fmsubadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fmsubadd_ps.html)
- [arch::x86::_mm_fnmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmadd_pd.html)
- [arch::x86::_mm_fnmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmadd_ps.html)
- [arch::x86::_mm_fnmadd_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmadd_sd.html)
- [arch::x86::_mm_fnmadd_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmadd_ss.html)
- [arch::x86::_mm_fnmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmsub_pd.html)
- [arch::x86::_mm_fnmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmsub_ps.html)
- [arch::x86::_mm_fnmsub_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmsub_sd.html)
- [arch::x86::_mm_fnmsub_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_fnmsub_ss.html)
- [arch::x86::_mm_getcsr](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_getcsr.html)
- [arch::x86::_mm_hadd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadd_epi16.html)
- [arch::x86::_mm_hadd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadd_epi32.html)
- [arch::x86::_mm_hadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadd_pd.html)
- [arch::x86::_mm_hadd_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadd_pi16.html)
- [arch::x86::_mm_hadd_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadd_pi32.html)
- [arch::x86::_mm_hadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadd_ps.html)
- [arch::x86::_mm_hadds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadds_epi16.html)
- [arch::x86::_mm_hadds_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hadds_pi16.html)
- [arch::x86::_mm_hsub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsub_epi16.html)
- [arch::x86::_mm_hsub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsub_epi32.html)
- [arch::x86::_mm_hsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsub_pd.html)
- [arch::x86::_mm_hsub_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsub_pi16.html)
- [arch::x86::_mm_hsub_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsub_pi32.html)
- [arch::x86::_mm_hsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsub_ps.html)
- [arch::x86::_mm_hsubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsubs_epi16.html)
- [arch::x86::_mm_hsubs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_hsubs_pi16.html)
- [arch::x86::_mm_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i32gather_epi32.html)
- [arch::x86::_mm_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i32gather_epi64.html)
- [arch::x86::_mm_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i32gather_pd.html)
- [arch::x86::_mm_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i32gather_ps.html)
- [arch::x86::_mm_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i64gather_epi32.html)
- [arch::x86::_mm_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i64gather_epi64.html)
- [arch::x86::_mm_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i64gather_pd.html)
- [arch::x86::_mm_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_i64gather_ps.html)
- [arch::x86::_mm_insert_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_insert_epi16.html)
- [arch::x86::_mm_insert_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_insert_epi32.html)
- [arch::x86::_mm_insert_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_insert_epi8.html)
- [arch::x86::_mm_insert_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_insert_pi16.html)
- [arch::x86::_mm_insert_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_insert_ps.html)
- [arch::x86::_mm_insert_si64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_insert_si64.html)
- [arch::x86::_mm_lddqu_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_lddqu_si128.html)
- [arch::x86::_mm_lfence](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_lfence.html)
- [arch::x86::_mm_load1_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load1_pd.html)
- [arch::x86::_mm_load1_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load1_ps.html)
- [arch::x86::_mm_load_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_pd.html)
- [arch::x86::_mm_load_pd1](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_pd1.html)
- [arch::x86::_mm_load_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_ps.html)
- [arch::x86::_mm_load_ps1](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_ps1.html)
- [arch::x86::_mm_load_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_sd.html)
- [arch::x86::_mm_load_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_si128.html)
- [arch::x86::_mm_load_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_load_ss.html)
- [arch::x86::_mm_loaddup_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loaddup_pd.html)
- [arch::x86::_mm_loadh_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadh_pd.html)
- [arch::x86::_mm_loadh_pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadh_pi.html)
- [arch::x86::_mm_loadl_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadl_epi64.html)
- [arch::x86::_mm_loadl_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadl_pd.html)
- [arch::x86::_mm_loadl_pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadl_pi.html)
- [arch::x86::_mm_loadr_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadr_pd.html)
- [arch::x86::_mm_loadr_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadr_ps.html)
- [arch::x86::_mm_loadu_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadu_pd.html)
- [arch::x86::_mm_loadu_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadu_ps.html)
- [arch::x86::_mm_loadu_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_loadu_si128.html)
- [arch::x86::_mm_madd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_madd_epi16.html)
- [arch::x86::_mm_maddubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maddubs_epi16.html)
- [arch::x86::_mm_maddubs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maddubs_pi16.html)
- [arch::x86::_mm_mask_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i32gather_epi32.html)
- [arch::x86::_mm_mask_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i32gather_epi64.html)
- [arch::x86::_mm_mask_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i32gather_pd.html)
- [arch::x86::_mm_mask_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i32gather_ps.html)
- [arch::x86::_mm_mask_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i64gather_epi32.html)
- [arch::x86::_mm_mask_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i64gather_epi64.html)
- [arch::x86::_mm_mask_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i64gather_pd.html)
- [arch::x86::_mm_mask_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mask_i64gather_ps.html)
- [arch::x86::_mm_maskload_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskload_epi32.html)
- [arch::x86::_mm_maskload_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskload_epi64.html)
- [arch::x86::_mm_maskload_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskload_pd.html)
- [arch::x86::_mm_maskload_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskload_ps.html)
- [arch::x86::_mm_maskmove_si64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskmove_si64.html)
- [arch::x86::_mm_maskmoveu_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskmoveu_si128.html)
- [arch::x86::_mm_maskstore_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskstore_epi32.html)
- [arch::x86::_mm_maskstore_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskstore_epi64.html)
- [arch::x86::_mm_maskstore_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskstore_pd.html)
- [arch::x86::_mm_maskstore_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_maskstore_ps.html)
- [arch::x86::_mm_max_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_epi16.html)
- [arch::x86::_mm_max_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_epi32.html)
- [arch::x86::_mm_max_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_epi8.html)
- [arch::x86::_mm_max_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_epu16.html)
- [arch::x86::_mm_max_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_epu32.html)
- [arch::x86::_mm_max_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_epu8.html)
- [arch::x86::_mm_max_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_pd.html)
- [arch::x86::_mm_max_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_pi16.html)
- [arch::x86::_mm_max_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_ps.html)
- [arch::x86::_mm_max_pu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_pu8.html)
- [arch::x86::_mm_max_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_sd.html)
- [arch::x86::_mm_max_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_max_ss.html)
- [arch::x86::_mm_mfence](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mfence.html)
- [arch::x86::_mm_min_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_epi16.html)
- [arch::x86::_mm_min_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_epi32.html)
- [arch::x86::_mm_min_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_epi8.html)
- [arch::x86::_mm_min_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_epu16.html)
- [arch::x86::_mm_min_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_epu32.html)
- [arch::x86::_mm_min_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_epu8.html)
- [arch::x86::_mm_min_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_pd.html)
- [arch::x86::_mm_min_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_pi16.html)
- [arch::x86::_mm_min_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_ps.html)
- [arch::x86::_mm_min_pu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_pu8.html)
- [arch::x86::_mm_min_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_sd.html)
- [arch::x86::_mm_min_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_min_ss.html)
- [arch::x86::_mm_minpos_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_minpos_epu16.html)
- [arch::x86::_mm_move_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_move_epi64.html)
- [arch::x86::_mm_move_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_move_sd.html)
- [arch::x86::_mm_move_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_move_ss.html)
- [arch::x86::_mm_movedup_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movedup_pd.html)
- [arch::x86::_mm_movehdup_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movehdup_ps.html)
- [arch::x86::_mm_movehl_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movehl_ps.html)
- [arch::x86::_mm_moveldup_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_moveldup_ps.html)
- [arch::x86::_mm_movelh_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movelh_ps.html)
- [arch::x86::_mm_movemask_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movemask_epi8.html)
- [arch::x86::_mm_movemask_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movemask_pd.html)
- [arch::x86::_mm_movemask_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movemask_pi8.html)
- [arch::x86::_mm_movemask_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movemask_ps.html)
- [arch::x86::_mm_movepi64_pi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movepi64_pi64.html)
- [arch::x86::_mm_movpi64_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_movpi64_epi64.html)
- [arch::x86::_mm_mpsadbw_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mpsadbw_epu8.html)
- [arch::x86::_mm_mul_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_epi32.html)
- [arch::x86::_mm_mul_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_epu32.html)
- [arch::x86::_mm_mul_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_pd.html)
- [arch::x86::_mm_mul_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_ps.html)
- [arch::x86::_mm_mul_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_sd.html)
- [arch::x86::_mm_mul_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_ss.html)
- [arch::x86::_mm_mul_su32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mul_su32.html)
- [arch::x86::_mm_mulhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mulhi_epi16.html)
- [arch::x86::_mm_mulhi_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mulhi_epu16.html)
- [arch::x86::_mm_mulhi_pu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mulhi_pu16.html)
- [arch::x86::_mm_mulhrs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mulhrs_epi16.html)
- [arch::x86::_mm_mulhrs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mulhrs_pi16.html)
- [arch::x86::_mm_mullo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mullo_epi16.html)
- [arch::x86::_mm_mullo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_mullo_epi32.html)
- [arch::x86::_mm_or_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_or_pd.html)
- [arch::x86::_mm_or_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_or_ps.html)
- [arch::x86::_mm_or_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_or_si128.html)
- [arch::x86::_mm_packs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_packs_epi16.html)
- [arch::x86::_mm_packs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_packs_epi32.html)
- [arch::x86::_mm_packs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_packs_pi16.html)
- [arch::x86::_mm_packs_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_packs_pi32.html)
- [arch::x86::_mm_packus_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_packus_epi16.html)
- [arch::x86::_mm_packus_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_packus_epi32.html)
- [arch::x86::_mm_pause](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_pause.html)
- [arch::x86::_mm_permute_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_permute_pd.html)
- [arch::x86::_mm_permute_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_permute_ps.html)
- [arch::x86::_mm_permutevar_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_permutevar_pd.html)
- [arch::x86::_mm_permutevar_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_permutevar_ps.html)
- [arch::x86::_mm_prefetch](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_prefetch.html)
- [arch::x86::_mm_rcp_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_rcp_ps.html)
- [arch::x86::_mm_rcp_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_rcp_ss.html)
- [arch::x86::_mm_round_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_round_pd.html)
- [arch::x86::_mm_round_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_round_ps.html)
- [arch::x86::_mm_round_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_round_sd.html)
- [arch::x86::_mm_round_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_round_ss.html)
- [arch::x86::_mm_rsqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_rsqrt_ps.html)
- [arch::x86::_mm_rsqrt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_rsqrt_ss.html)
- [arch::x86::_mm_sad_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sad_epu8.html)
- [arch::x86::_mm_sad_pu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sad_pu8.html)
- [arch::x86::_mm_set1_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_epi16.html)
- [arch::x86::_mm_set1_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_epi32.html)
- [arch::x86::_mm_set1_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_epi64.html)
- [arch::x86::_mm_set1_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_epi64x.html)
- [arch::x86::_mm_set1_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_epi8.html)
- [arch::x86::_mm_set1_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_pd.html)
- [arch::x86::_mm_set1_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_pi16.html)
- [arch::x86::_mm_set1_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_pi32.html)
- [arch::x86::_mm_set1_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_pi8.html)
- [arch::x86::_mm_set1_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set1_ps.html)
- [arch::x86::_mm_set_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_epi16.html)
- [arch::x86::_mm_set_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_epi32.html)
- [arch::x86::_mm_set_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_epi64.html)
- [arch::x86::_mm_set_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_epi64x.html)
- [arch::x86::_mm_set_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_epi8.html)
- [arch::x86::_mm_set_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_pd.html)
- [arch::x86::_mm_set_pd1](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_pd1.html)
- [arch::x86::_mm_set_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_pi16.html)
- [arch::x86::_mm_set_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_pi32.html)
- [arch::x86::_mm_set_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_pi8.html)
- [arch::x86::_mm_set_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_ps.html)
- [arch::x86::_mm_set_ps1](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_ps1.html)
- [arch::x86::_mm_set_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_sd.html)
- [arch::x86::_mm_set_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_set_ss.html)
- [arch::x86::_mm_setcsr](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setcsr.html)
- [arch::x86::_mm_setr_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_epi16.html)
- [arch::x86::_mm_setr_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_epi32.html)
- [arch::x86::_mm_setr_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_epi64.html)
- [arch::x86::_mm_setr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_epi8.html)
- [arch::x86::_mm_setr_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_pd.html)
- [arch::x86::_mm_setr_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_pi16.html)
- [arch::x86::_mm_setr_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_pi32.html)
- [arch::x86::_mm_setr_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_pi8.html)
- [arch::x86::_mm_setr_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setr_ps.html)
- [arch::x86::_mm_setzero_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setzero_pd.html)
- [arch::x86::_mm_setzero_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setzero_ps.html)
- [arch::x86::_mm_setzero_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setzero_si128.html)
- [arch::x86::_mm_setzero_si64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_setzero_si64.html)
- [arch::x86::_mm_sfence](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sfence.html)
- [arch::x86::_mm_sha1msg1_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha1msg1_epu32.html)
- [arch::x86::_mm_sha1msg2_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha1msg2_epu32.html)
- [arch::x86::_mm_sha1nexte_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha1nexte_epu32.html)
- [arch::x86::_mm_sha1rnds4_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha1rnds4_epu32.html)
- [arch::x86::_mm_sha256msg1_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha256msg1_epu32.html)
- [arch::x86::_mm_sha256msg2_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha256msg2_epu32.html)
- [arch::x86::_mm_sha256rnds2_epu32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sha256rnds2_epu32.html)
- [arch::x86::_mm_shuffle_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shuffle_epi32.html)
- [arch::x86::_mm_shuffle_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shuffle_epi8.html)
- [arch::x86::_mm_shuffle_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shuffle_pd.html)
- [arch::x86::_mm_shuffle_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shuffle_pi16.html)
- [arch::x86::_mm_shuffle_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shuffle_pi8.html)
- [arch::x86::_mm_shuffle_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shuffle_ps.html)
- [arch::x86::_mm_shufflehi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shufflehi_epi16.html)
- [arch::x86::_mm_shufflelo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_shufflelo_epi16.html)
- [arch::x86::_mm_sign_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sign_epi16.html)
- [arch::x86::_mm_sign_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sign_epi32.html)
- [arch::x86::_mm_sign_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sign_epi8.html)
- [arch::x86::_mm_sign_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sign_pi16.html)
- [arch::x86::_mm_sign_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sign_pi32.html)
- [arch::x86::_mm_sign_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sign_pi8.html)
- [arch::x86::_mm_sll_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sll_epi16.html)
- [arch::x86::_mm_sll_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sll_epi32.html)
- [arch::x86::_mm_sll_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sll_epi64.html)
- [arch::x86::_mm_slli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_slli_epi16.html)
- [arch::x86::_mm_slli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_slli_epi32.html)
- [arch::x86::_mm_slli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_slli_epi64.html)
- [arch::x86::_mm_slli_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_slli_si128.html)
- [arch::x86::_mm_sllv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sllv_epi32.html)
- [arch::x86::_mm_sllv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sllv_epi64.html)
- [arch::x86::_mm_sqrt_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sqrt_pd.html)
- [arch::x86::_mm_sqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sqrt_ps.html)
- [arch::x86::_mm_sqrt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sqrt_sd.html)
- [arch::x86::_mm_sqrt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sqrt_ss.html)
- [arch::x86::_mm_sra_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sra_epi16.html)
- [arch::x86::_mm_sra_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sra_epi32.html)
- [arch::x86::_mm_srai_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srai_epi16.html)
- [arch::x86::_mm_srai_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srai_epi32.html)
- [arch::x86::_mm_srav_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srav_epi32.html)
- [arch::x86::_mm_srl_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srl_epi16.html)
- [arch::x86::_mm_srl_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srl_epi32.html)
- [arch::x86::_mm_srl_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srl_epi64.html)
- [arch::x86::_mm_srli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srli_epi16.html)
- [arch::x86::_mm_srli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srli_epi32.html)
- [arch::x86::_mm_srli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srli_epi64.html)
- [arch::x86::_mm_srli_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srli_si128.html)
- [arch::x86::_mm_srlv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srlv_epi32.html)
- [arch::x86::_mm_srlv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_srlv_epi64.html)
- [arch::x86::_mm_store1_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store1_pd.html)
- [arch::x86::_mm_store1_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store1_ps.html)
- [arch::x86::_mm_store_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_pd.html)
- [arch::x86::_mm_store_pd1](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_pd1.html)
- [arch::x86::_mm_store_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_ps.html)
- [arch::x86::_mm_store_ps1](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_ps1.html)
- [arch::x86::_mm_store_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_sd.html)
- [arch::x86::_mm_store_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_si128.html)
- [arch::x86::_mm_store_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_store_ss.html)
- [arch::x86::_mm_storeh_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storeh_pd.html)
- [arch::x86::_mm_storeh_pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storeh_pi.html)
- [arch::x86::_mm_storel_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storel_epi64.html)
- [arch::x86::_mm_storel_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storel_pd.html)
- [arch::x86::_mm_storel_pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storel_pi.html)
- [arch::x86::_mm_storer_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storer_pd.html)
- [arch::x86::_mm_storer_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storer_ps.html)
- [arch::x86::_mm_storeu_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storeu_pd.html)
- [arch::x86::_mm_storeu_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storeu_ps.html)
- [arch::x86::_mm_storeu_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_storeu_si128.html)
- [arch::x86::_mm_stream_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_pd.html)
- [arch::x86::_mm_stream_pi](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_pi.html)
- [arch::x86::_mm_stream_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_ps.html)
- [arch::x86::_mm_stream_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_sd.html)
- [arch::x86::_mm_stream_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_si128.html)
- [arch::x86::_mm_stream_si32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_si32.html)
- [arch::x86::_mm_stream_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_stream_ss.html)
- [arch::x86::_mm_sub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_epi16.html)
- [arch::x86::_mm_sub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_epi32.html)
- [arch::x86::_mm_sub_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_epi64.html)
- [arch::x86::_mm_sub_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_epi8.html)
- [arch::x86::_mm_sub_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_pd.html)
- [arch::x86::_mm_sub_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_pi16.html)
- [arch::x86::_mm_sub_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_pi32.html)
- [arch::x86::_mm_sub_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_pi8.html)
- [arch::x86::_mm_sub_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_ps.html)
- [arch::x86::_mm_sub_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_sd.html)
- [arch::x86::_mm_sub_si64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_si64.html)
- [arch::x86::_mm_sub_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_sub_ss.html)
- [arch::x86::_mm_subs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_epi16.html)
- [arch::x86::_mm_subs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_epi8.html)
- [arch::x86::_mm_subs_epu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_epu16.html)
- [arch::x86::_mm_subs_epu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_epu8.html)
- [arch::x86::_mm_subs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_pi16.html)
- [arch::x86::_mm_subs_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_pi8.html)
- [arch::x86::_mm_subs_pu16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_pu16.html)
- [arch::x86::_mm_subs_pu8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_subs_pu8.html)
- [arch::x86::_mm_test_all_ones](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_test_all_ones.html)
- [arch::x86::_mm_test_all_zeros](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_test_all_zeros.html)
- [arch::x86::_mm_test_mix_ones_zeros](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_test_mix_ones_zeros.html)
- [arch::x86::_mm_testc_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testc_pd.html)
- [arch::x86::_mm_testc_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testc_ps.html)
- [arch::x86::_mm_testc_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testc_si128.html)
- [arch::x86::_mm_testnzc_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testnzc_pd.html)
- [arch::x86::_mm_testnzc_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testnzc_ps.html)
- [arch::x86::_mm_testnzc_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testnzc_si128.html)
- [arch::x86::_mm_testz_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testz_pd.html)
- [arch::x86::_mm_testz_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testz_ps.html)
- [arch::x86::_mm_testz_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_testz_si128.html)
- [arch::x86::_mm_tzcnt_32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_tzcnt_32.html)
- [arch::x86::_mm_ucomieq_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomieq_sd.html)
- [arch::x86::_mm_ucomieq_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomieq_ss.html)
- [arch::x86::_mm_ucomige_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomige_sd.html)
- [arch::x86::_mm_ucomige_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomige_ss.html)
- [arch::x86::_mm_ucomigt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomigt_sd.html)
- [arch::x86::_mm_ucomigt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomigt_ss.html)
- [arch::x86::_mm_ucomile_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomile_sd.html)
- [arch::x86::_mm_ucomile_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomile_ss.html)
- [arch::x86::_mm_ucomilt_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomilt_sd.html)
- [arch::x86::_mm_ucomilt_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomilt_ss.html)
- [arch::x86::_mm_ucomineq_sd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomineq_sd.html)
- [arch::x86::_mm_ucomineq_ss](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_ucomineq_ss.html)
- [arch::x86::_mm_undefined_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_undefined_pd.html)
- [arch::x86::_mm_undefined_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_undefined_ps.html)
- [arch::x86::_mm_undefined_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_undefined_si128.html)
- [arch::x86::_mm_unpackhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_epi16.html)
- [arch::x86::_mm_unpackhi_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_epi32.html)
- [arch::x86::_mm_unpackhi_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_epi64.html)
- [arch::x86::_mm_unpackhi_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_epi8.html)
- [arch::x86::_mm_unpackhi_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_pd.html)
- [arch::x86::_mm_unpackhi_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_pi16.html)
- [arch::x86::_mm_unpackhi_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_pi32.html)
- [arch::x86::_mm_unpackhi_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_pi8.html)
- [arch::x86::_mm_unpackhi_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpackhi_ps.html)
- [arch::x86::_mm_unpacklo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_epi16.html)
- [arch::x86::_mm_unpacklo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_epi32.html)
- [arch::x86::_mm_unpacklo_epi64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_epi64.html)
- [arch::x86::_mm_unpacklo_epi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_epi8.html)
- [arch::x86::_mm_unpacklo_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_pd.html)
- [arch::x86::_mm_unpacklo_pi16](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_pi16.html)
- [arch::x86::_mm_unpacklo_pi32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_pi32.html)
- [arch::x86::_mm_unpacklo_pi8](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_pi8.html)
- [arch::x86::_mm_unpacklo_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_unpacklo_ps.html)
- [arch::x86::_mm_xor_pd](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_xor_pd.html)
- [arch::x86::_mm_xor_ps](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_xor_ps.html)
- [arch::x86::_mm_xor_si128](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mm_xor_si128.html)
- [arch::x86::_mulx_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._mulx_u32.html)
- [arch::x86::_pdep_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._pdep_u32.html)
- [arch::x86::_pext_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._pext_u32.html)
- [arch::x86::_popcnt32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._popcnt32.html)
- [arch::x86::_rdrand16_step](https://doc.rust-lang.org/nightly/core/arch/x86/fn._rdrand16_step.html)
- [arch::x86::_rdrand32_step](https://doc.rust-lang.org/nightly/core/arch/x86/fn._rdrand32_step.html)
- [arch::x86::_rdseed16_step](https://doc.rust-lang.org/nightly/core/arch/x86/fn._rdseed16_step.html)
- [arch::x86::_rdseed32_step](https://doc.rust-lang.org/nightly/core/arch/x86/fn._rdseed32_step.html)
- [arch::x86::_rdtsc](https://doc.rust-lang.org/nightly/core/arch/x86/fn._rdtsc.html)
- [arch::x86::_t1mskc_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._t1mskc_u32.html)
- [arch::x86::_t1mskc_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._t1mskc_u64.html)
- [arch::x86::_tzcnt_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._tzcnt_u32.html)
- [arch::x86::_tzmsk_u32](https://doc.rust-lang.org/nightly/core/arch/x86/fn._tzmsk_u32.html)
- [arch::x86::_tzmsk_u64](https://doc.rust-lang.org/nightly/core/arch/x86/fn._tzmsk_u64.html)
- [arch::x86::_xgetbv](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xgetbv.html)
- [arch::x86::_xrstor](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xrstor.html)
- [arch::x86::_xrstors](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xrstors.html)
- [arch::x86::_xsave](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xsave.html)
- [arch::x86::_xsavec](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xsavec.html)
- [arch::x86::_xsaveopt](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xsaveopt.html)
- [arch::x86::_xsaves](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xsaves.html)
- [arch::x86::_xsetbv](https://doc.rust-lang.org/nightly/core/arch/x86/fn._xsetbv.html)
- [arch::x86::has_cpuid](https://doc.rust-lang.org/nightly/core/arch/x86/fn.has_cpuid.html)
- [arch::x86_64::_MM_GET_EXCEPTION_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_GET_EXCEPTION_MASK.html)
- [arch::x86_64::_MM_GET_EXCEPTION_STATE](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_GET_EXCEPTION_STATE.html)
- [arch::x86_64::_MM_GET_FLUSH_ZERO_MODE](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_GET_FLUSH_ZERO_MODE.html)
- [arch::x86_64::_MM_GET_ROUNDING_MODE](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_GET_ROUNDING_MODE.html)
- [arch::x86_64::_MM_SET_EXCEPTION_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_SET_EXCEPTION_MASK.html)
- [arch::x86_64::_MM_SET_EXCEPTION_STATE](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_SET_EXCEPTION_STATE.html)
- [arch::x86_64::_MM_SET_FLUSH_ZERO_MODE](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_SET_FLUSH_ZERO_MODE.html)
- [arch::x86_64::_MM_SET_ROUNDING_MODE](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_SET_ROUNDING_MODE.html)
- [arch::x86_64::_MM_TRANSPOSE4_PS](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._MM_TRANSPOSE4_PS.html)
- [arch::x86_64::__cpuid](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.__cpuid.html)
- [arch::x86_64::__cpuid_count](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.__cpuid_count.html)
- [arch::x86_64::__get_cpuid_max](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.__get_cpuid_max.html)
- [arch::x86_64::__rdtscp](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.__rdtscp.html)
- [arch::x86_64::__readeflags](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.__readeflags.html)
- [arch::x86_64::__writeeflags](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.__writeeflags.html)
- [arch::x86_64::_andn_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._andn_u32.html)
- [arch::x86_64::_andn_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._andn_u64.html)
- [arch::x86_64::_bextr2_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bextr2_u32.html)
- [arch::x86_64::_bextr2_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bextr2_u64.html)
- [arch::x86_64::_bextr_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bextr_u32.html)
- [arch::x86_64::_bextr_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bextr_u64.html)
- [arch::x86_64::_blcfill_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcfill_u32.html)
- [arch::x86_64::_blcfill_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcfill_u64.html)
- [arch::x86_64::_blci_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blci_u32.html)
- [arch::x86_64::_blci_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blci_u64.html)
- [arch::x86_64::_blcic_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcic_u32.html)
- [arch::x86_64::_blcic_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcic_u64.html)
- [arch::x86_64::_blcmsk_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcmsk_u32.html)
- [arch::x86_64::_blcmsk_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcmsk_u64.html)
- [arch::x86_64::_blcs_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcs_u32.html)
- [arch::x86_64::_blcs_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blcs_u64.html)
- [arch::x86_64::_blsfill_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsfill_u32.html)
- [arch::x86_64::_blsfill_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsfill_u64.html)
- [arch::x86_64::_blsi_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsi_u32.html)
- [arch::x86_64::_blsi_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsi_u64.html)
- [arch::x86_64::_blsic_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsic_u32.html)
- [arch::x86_64::_blsic_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsic_u64.html)
- [arch::x86_64::_blsmsk_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsmsk_u32.html)
- [arch::x86_64::_blsmsk_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsmsk_u64.html)
- [arch::x86_64::_blsr_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsr_u32.html)
- [arch::x86_64::_blsr_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._blsr_u64.html)
- [arch::x86_64::_bswap](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bswap.html)
- [arch::x86_64::_bswap64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bswap64.html)
- [arch::x86_64::_bzhi_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bzhi_u32.html)
- [arch::x86_64::_bzhi_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._bzhi_u64.html)
- [arch::x86_64::_fxrstor](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._fxrstor.html)
- [arch::x86_64::_fxrstor64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._fxrstor64.html)
- [arch::x86_64::_fxsave](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._fxsave.html)
- [arch::x86_64::_fxsave64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._fxsave64.html)
- [arch::x86_64::_lzcnt_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._lzcnt_u32.html)
- [arch::x86_64::_lzcnt_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._lzcnt_u64.html)
- [arch::x86_64::_m_maskmovq](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_maskmovq.html)
- [arch::x86_64::_m_paddb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddb.html)
- [arch::x86_64::_m_paddd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddd.html)
- [arch::x86_64::_m_paddsb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddsb.html)
- [arch::x86_64::_m_paddsw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddsw.html)
- [arch::x86_64::_m_paddusb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddusb.html)
- [arch::x86_64::_m_paddusw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddusw.html)
- [arch::x86_64::_m_paddw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_paddw.html)
- [arch::x86_64::_m_pavgb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pavgb.html)
- [arch::x86_64::_m_pavgw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pavgw.html)
- [arch::x86_64::_m_pextrw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pextrw.html)
- [arch::x86_64::_m_pinsrw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pinsrw.html)
- [arch::x86_64::_m_pmaxsw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pmaxsw.html)
- [arch::x86_64::_m_pmaxub](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pmaxub.html)
- [arch::x86_64::_m_pminsw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pminsw.html)
- [arch::x86_64::_m_pminub](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pminub.html)
- [arch::x86_64::_m_pmovmskb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pmovmskb.html)
- [arch::x86_64::_m_pmulhuw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pmulhuw.html)
- [arch::x86_64::_m_psadbw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psadbw.html)
- [arch::x86_64::_m_pshufw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_pshufw.html)
- [arch::x86_64::_m_psubb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubb.html)
- [arch::x86_64::_m_psubd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubd.html)
- [arch::x86_64::_m_psubsb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubsb.html)
- [arch::x86_64::_m_psubsw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubsw.html)
- [arch::x86_64::_m_psubusb](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubusb.html)
- [arch::x86_64::_m_psubusw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubusw.html)
- [arch::x86_64::_m_psubw](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._m_psubw.html)
- [arch::x86_64::_mm256_abs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_abs_epi16.html)
- [arch::x86_64::_mm256_abs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_abs_epi32.html)
- [arch::x86_64::_mm256_abs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_abs_epi8.html)
- [arch::x86_64::_mm256_add_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_add_epi16.html)
- [arch::x86_64::_mm256_add_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_add_epi32.html)
- [arch::x86_64::_mm256_add_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_add_epi64.html)
- [arch::x86_64::_mm256_add_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_add_epi8.html)
- [arch::x86_64::_mm256_add_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_add_pd.html)
- [arch::x86_64::_mm256_add_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_add_ps.html)
- [arch::x86_64::_mm256_adds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_adds_epi16.html)
- [arch::x86_64::_mm256_adds_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_adds_epi8.html)
- [arch::x86_64::_mm256_adds_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_adds_epu16.html)
- [arch::x86_64::_mm256_adds_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_adds_epu8.html)
- [arch::x86_64::_mm256_addsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_addsub_pd.html)
- [arch::x86_64::_mm256_addsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_addsub_ps.html)
- [arch::x86_64::_mm256_alignr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_alignr_epi8.html)
- [arch::x86_64::_mm256_and_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_and_pd.html)
- [arch::x86_64::_mm256_and_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_and_ps.html)
- [arch::x86_64::_mm256_and_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_and_si256.html)
- [arch::x86_64::_mm256_andnot_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_andnot_pd.html)
- [arch::x86_64::_mm256_andnot_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_andnot_ps.html)
- [arch::x86_64::_mm256_andnot_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_andnot_si256.html)
- [arch::x86_64::_mm256_avg_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_avg_epu16.html)
- [arch::x86_64::_mm256_avg_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_avg_epu8.html)
- [arch::x86_64::_mm256_blend_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blend_epi16.html)
- [arch::x86_64::_mm256_blend_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blend_epi32.html)
- [arch::x86_64::_mm256_blend_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blend_pd.html)
- [arch::x86_64::_mm256_blend_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blend_ps.html)
- [arch::x86_64::_mm256_blendv_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blendv_epi8.html)
- [arch::x86_64::_mm256_blendv_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blendv_pd.html)
- [arch::x86_64::_mm256_blendv_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_blendv_ps.html)
- [arch::x86_64::_mm256_broadcast_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcast_pd.html)
- [arch::x86_64::_mm256_broadcast_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcast_ps.html)
- [arch::x86_64::_mm256_broadcast_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcast_sd.html)
- [arch::x86_64::_mm256_broadcast_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcast_ss.html)
- [arch::x86_64::_mm256_broadcastb_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastb_epi8.html)
- [arch::x86_64::_mm256_broadcastd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastd_epi32.html)
- [arch::x86_64::_mm256_broadcastq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastq_epi64.html)
- [arch::x86_64::_mm256_broadcastsd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastsd_pd.html)
- [arch::x86_64::_mm256_broadcastsi128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastsi128_si256.html)
- [arch::x86_64::_mm256_broadcastss_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastss_ps.html)
- [arch::x86_64::_mm256_broadcastw_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_broadcastw_epi16.html)
- [arch::x86_64::_mm256_bslli_epi128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_bslli_epi128.html)
- [arch::x86_64::_mm256_bsrli_epi128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_bsrli_epi128.html)
- [arch::x86_64::_mm256_castpd128_pd256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castpd128_pd256.html)
- [arch::x86_64::_mm256_castpd256_pd128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castpd256_pd128.html)
- [arch::x86_64::_mm256_castpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castpd_ps.html)
- [arch::x86_64::_mm256_castpd_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castpd_si256.html)
- [arch::x86_64::_mm256_castps128_ps256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castps128_ps256.html)
- [arch::x86_64::_mm256_castps256_ps128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castps256_ps128.html)
- [arch::x86_64::_mm256_castps_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castps_pd.html)
- [arch::x86_64::_mm256_castps_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castps_si256.html)
- [arch::x86_64::_mm256_castsi128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castsi128_si256.html)
- [arch::x86_64::_mm256_castsi256_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castsi256_pd.html)
- [arch::x86_64::_mm256_castsi256_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castsi256_ps.html)
- [arch::x86_64::_mm256_castsi256_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_castsi256_si128.html)
- [arch::x86_64::_mm256_ceil_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_ceil_pd.html)
- [arch::x86_64::_mm256_ceil_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_ceil_ps.html)
- [arch::x86_64::_mm256_cmp_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmp_pd.html)
- [arch::x86_64::_mm256_cmp_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmp_ps.html)
- [arch::x86_64::_mm256_cmpeq_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpeq_epi16.html)
- [arch::x86_64::_mm256_cmpeq_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpeq_epi32.html)
- [arch::x86_64::_mm256_cmpeq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpeq_epi64.html)
- [arch::x86_64::_mm256_cmpeq_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpeq_epi8.html)
- [arch::x86_64::_mm256_cmpgt_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpgt_epi16.html)
- [arch::x86_64::_mm256_cmpgt_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpgt_epi32.html)
- [arch::x86_64::_mm256_cmpgt_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpgt_epi64.html)
- [arch::x86_64::_mm256_cmpgt_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cmpgt_epi8.html)
- [arch::x86_64::_mm256_cvtepi16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi16_epi32.html)
- [arch::x86_64::_mm256_cvtepi16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi16_epi64.html)
- [arch::x86_64::_mm256_cvtepi32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi32_epi64.html)
- [arch::x86_64::_mm256_cvtepi32_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi32_pd.html)
- [arch::x86_64::_mm256_cvtepi32_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi32_ps.html)
- [arch::x86_64::_mm256_cvtepi8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi8_epi16.html)
- [arch::x86_64::_mm256_cvtepi8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi8_epi32.html)
- [arch::x86_64::_mm256_cvtepi8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepi8_epi64.html)
- [arch::x86_64::_mm256_cvtepu16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepu16_epi32.html)
- [arch::x86_64::_mm256_cvtepu16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepu16_epi64.html)
- [arch::x86_64::_mm256_cvtepu32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepu32_epi64.html)
- [arch::x86_64::_mm256_cvtepu8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepu8_epi16.html)
- [arch::x86_64::_mm256_cvtepu8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepu8_epi32.html)
- [arch::x86_64::_mm256_cvtepu8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtepu8_epi64.html)
- [arch::x86_64::_mm256_cvtpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtpd_epi32.html)
- [arch::x86_64::_mm256_cvtpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtpd_ps.html)
- [arch::x86_64::_mm256_cvtps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtps_epi32.html)
- [arch::x86_64::_mm256_cvtps_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtps_pd.html)
- [arch::x86_64::_mm256_cvtsd_f64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtsd_f64.html)
- [arch::x86_64::_mm256_cvtsi256_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtsi256_si32.html)
- [arch::x86_64::_mm256_cvtss_f32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvtss_f32.html)
- [arch::x86_64::_mm256_cvttpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvttpd_epi32.html)
- [arch::x86_64::_mm256_cvttps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_cvttps_epi32.html)
- [arch::x86_64::_mm256_div_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_div_pd.html)
- [arch::x86_64::_mm256_div_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_div_ps.html)
- [arch::x86_64::_mm256_dp_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_dp_ps.html)
- [arch::x86_64::_mm256_extract_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extract_epi16.html)
- [arch::x86_64::_mm256_extract_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extract_epi32.html)
- [arch::x86_64::_mm256_extract_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extract_epi64.html)
- [arch::x86_64::_mm256_extract_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extract_epi8.html)
- [arch::x86_64::_mm256_extractf128_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extractf128_pd.html)
- [arch::x86_64::_mm256_extractf128_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extractf128_ps.html)
- [arch::x86_64::_mm256_extractf128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extractf128_si256.html)
- [arch::x86_64::_mm256_extracti128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_extracti128_si256.html)
- [arch::x86_64::_mm256_floor_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_floor_pd.html)
- [arch::x86_64::_mm256_floor_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_floor_ps.html)
- [arch::x86_64::_mm256_fmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmadd_pd.html)
- [arch::x86_64::_mm256_fmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmadd_ps.html)
- [arch::x86_64::_mm256_fmaddsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmaddsub_pd.html)
- [arch::x86_64::_mm256_fmaddsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmaddsub_ps.html)
- [arch::x86_64::_mm256_fmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmsub_pd.html)
- [arch::x86_64::_mm256_fmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmsub_ps.html)
- [arch::x86_64::_mm256_fmsubadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmsubadd_pd.html)
- [arch::x86_64::_mm256_fmsubadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fmsubadd_ps.html)
- [arch::x86_64::_mm256_fnmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fnmadd_pd.html)
- [arch::x86_64::_mm256_fnmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fnmadd_ps.html)
- [arch::x86_64::_mm256_fnmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fnmsub_pd.html)
- [arch::x86_64::_mm256_fnmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_fnmsub_ps.html)
- [arch::x86_64::_mm256_hadd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hadd_epi16.html)
- [arch::x86_64::_mm256_hadd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hadd_epi32.html)
- [arch::x86_64::_mm256_hadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hadd_pd.html)
- [arch::x86_64::_mm256_hadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hadd_ps.html)
- [arch::x86_64::_mm256_hadds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hadds_epi16.html)
- [arch::x86_64::_mm256_hsub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hsub_epi16.html)
- [arch::x86_64::_mm256_hsub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hsub_epi32.html)
- [arch::x86_64::_mm256_hsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hsub_pd.html)
- [arch::x86_64::_mm256_hsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hsub_ps.html)
- [arch::x86_64::_mm256_hsubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_hsubs_epi16.html)
- [arch::x86_64::_mm256_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i32gather_epi32.html)
- [arch::x86_64::_mm256_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i32gather_epi64.html)
- [arch::x86_64::_mm256_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i32gather_pd.html)
- [arch::x86_64::_mm256_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i32gather_ps.html)
- [arch::x86_64::_mm256_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i64gather_epi32.html)
- [arch::x86_64::_mm256_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i64gather_epi64.html)
- [arch::x86_64::_mm256_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i64gather_pd.html)
- [arch::x86_64::_mm256_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_i64gather_ps.html)
- [arch::x86_64::_mm256_insert_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insert_epi16.html)
- [arch::x86_64::_mm256_insert_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insert_epi32.html)
- [arch::x86_64::_mm256_insert_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insert_epi64.html)
- [arch::x86_64::_mm256_insert_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insert_epi8.html)
- [arch::x86_64::_mm256_insertf128_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insertf128_pd.html)
- [arch::x86_64::_mm256_insertf128_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insertf128_ps.html)
- [arch::x86_64::_mm256_insertf128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_insertf128_si256.html)
- [arch::x86_64::_mm256_inserti128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_inserti128_si256.html)
- [arch::x86_64::_mm256_lddqu_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_lddqu_si256.html)
- [arch::x86_64::_mm256_load_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_load_pd.html)
- [arch::x86_64::_mm256_load_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_load_ps.html)
- [arch::x86_64::_mm256_load_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_load_si256.html)
- [arch::x86_64::_mm256_loadu2_m128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_loadu2_m128.html)
- [arch::x86_64::_mm256_loadu2_m128d](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_loadu2_m128d.html)
- [arch::x86_64::_mm256_loadu2_m128i](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_loadu2_m128i.html)
- [arch::x86_64::_mm256_loadu_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_loadu_pd.html)
- [arch::x86_64::_mm256_loadu_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_loadu_ps.html)
- [arch::x86_64::_mm256_loadu_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_loadu_si256.html)
- [arch::x86_64::_mm256_madd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_madd_epi16.html)
- [arch::x86_64::_mm256_maddubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maddubs_epi16.html)
- [arch::x86_64::_mm256_mask_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i32gather_epi32.html)
- [arch::x86_64::_mm256_mask_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i32gather_epi64.html)
- [arch::x86_64::_mm256_mask_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i32gather_pd.html)
- [arch::x86_64::_mm256_mask_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i32gather_ps.html)
- [arch::x86_64::_mm256_mask_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i64gather_epi32.html)
- [arch::x86_64::_mm256_mask_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i64gather_epi64.html)
- [arch::x86_64::_mm256_mask_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i64gather_pd.html)
- [arch::x86_64::_mm256_mask_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mask_i64gather_ps.html)
- [arch::x86_64::_mm256_maskload_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskload_epi32.html)
- [arch::x86_64::_mm256_maskload_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskload_epi64.html)
- [arch::x86_64::_mm256_maskload_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskload_pd.html)
- [arch::x86_64::_mm256_maskload_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskload_ps.html)
- [arch::x86_64::_mm256_maskstore_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskstore_epi32.html)
- [arch::x86_64::_mm256_maskstore_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskstore_epi64.html)
- [arch::x86_64::_mm256_maskstore_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskstore_pd.html)
- [arch::x86_64::_mm256_maskstore_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_maskstore_ps.html)
- [arch::x86_64::_mm256_max_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_epi16.html)
- [arch::x86_64::_mm256_max_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_epi32.html)
- [arch::x86_64::_mm256_max_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_epi8.html)
- [arch::x86_64::_mm256_max_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_epu16.html)
- [arch::x86_64::_mm256_max_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_epu32.html)
- [arch::x86_64::_mm256_max_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_epu8.html)
- [arch::x86_64::_mm256_max_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_pd.html)
- [arch::x86_64::_mm256_max_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_max_ps.html)
- [arch::x86_64::_mm256_min_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_epi16.html)
- [arch::x86_64::_mm256_min_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_epi32.html)
- [arch::x86_64::_mm256_min_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_epi8.html)
- [arch::x86_64::_mm256_min_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_epu16.html)
- [arch::x86_64::_mm256_min_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_epu32.html)
- [arch::x86_64::_mm256_min_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_epu8.html)
- [arch::x86_64::_mm256_min_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_pd.html)
- [arch::x86_64::_mm256_min_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_min_ps.html)
- [arch::x86_64::_mm256_movedup_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_movedup_pd.html)
- [arch::x86_64::_mm256_movehdup_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_movehdup_ps.html)
- [arch::x86_64::_mm256_moveldup_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_moveldup_ps.html)
- [arch::x86_64::_mm256_movemask_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_movemask_epi8.html)
- [arch::x86_64::_mm256_movemask_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_movemask_pd.html)
- [arch::x86_64::_mm256_movemask_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_movemask_ps.html)
- [arch::x86_64::_mm256_mpsadbw_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mpsadbw_epu8.html)
- [arch::x86_64::_mm256_mul_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mul_epi32.html)
- [arch::x86_64::_mm256_mul_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mul_epu32.html)
- [arch::x86_64::_mm256_mul_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mul_pd.html)
- [arch::x86_64::_mm256_mul_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mul_ps.html)
- [arch::x86_64::_mm256_mulhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mulhi_epi16.html)
- [arch::x86_64::_mm256_mulhi_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mulhi_epu16.html)
- [arch::x86_64::_mm256_mulhrs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mulhrs_epi16.html)
- [arch::x86_64::_mm256_mullo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mullo_epi16.html)
- [arch::x86_64::_mm256_mullo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_mullo_epi32.html)
- [arch::x86_64::_mm256_or_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_or_pd.html)
- [arch::x86_64::_mm256_or_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_or_ps.html)
- [arch::x86_64::_mm256_or_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_or_si256.html)
- [arch::x86_64::_mm256_packs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_packs_epi16.html)
- [arch::x86_64::_mm256_packs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_packs_epi32.html)
- [arch::x86_64::_mm256_packus_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_packus_epi16.html)
- [arch::x86_64::_mm256_packus_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_packus_epi32.html)
- [arch::x86_64::_mm256_permute2f128_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute2f128_pd.html)
- [arch::x86_64::_mm256_permute2f128_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute2f128_ps.html)
- [arch::x86_64::_mm256_permute2f128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute2f128_si256.html)
- [arch::x86_64::_mm256_permute2x128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute2x128_si256.html)
- [arch::x86_64::_mm256_permute4x64_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute4x64_epi64.html)
- [arch::x86_64::_mm256_permute4x64_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute4x64_pd.html)
- [arch::x86_64::_mm256_permute_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute_pd.html)
- [arch::x86_64::_mm256_permute_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permute_ps.html)
- [arch::x86_64::_mm256_permutevar8x32_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permutevar8x32_epi32.html)
- [arch::x86_64::_mm256_permutevar8x32_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permutevar8x32_ps.html)
- [arch::x86_64::_mm256_permutevar_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permutevar_pd.html)
- [arch::x86_64::_mm256_permutevar_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_permutevar_ps.html)
- [arch::x86_64::_mm256_rcp_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_rcp_ps.html)
- [arch::x86_64::_mm256_round_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_round_pd.html)
- [arch::x86_64::_mm256_round_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_round_ps.html)
- [arch::x86_64::_mm256_rsqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_rsqrt_ps.html)
- [arch::x86_64::_mm256_sad_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sad_epu8.html)
- [arch::x86_64::_mm256_set1_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set1_epi16.html)
- [arch::x86_64::_mm256_set1_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set1_epi32.html)
- [arch::x86_64::_mm256_set1_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set1_epi64x.html)
- [arch::x86_64::_mm256_set1_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set1_epi8.html)
- [arch::x86_64::_mm256_set1_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set1_pd.html)
- [arch::x86_64::_mm256_set1_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set1_ps.html)
- [arch::x86_64::_mm256_set_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_epi16.html)
- [arch::x86_64::_mm256_set_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_epi32.html)
- [arch::x86_64::_mm256_set_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_epi64x.html)
- [arch::x86_64::_mm256_set_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_epi8.html)
- [arch::x86_64::_mm256_set_m128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_m128.html)
- [arch::x86_64::_mm256_set_m128d](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_m128d.html)
- [arch::x86_64::_mm256_set_m128i](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_m128i.html)
- [arch::x86_64::_mm256_set_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_pd.html)
- [arch::x86_64::_mm256_set_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_set_ps.html)
- [arch::x86_64::_mm256_setr_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_epi16.html)
- [arch::x86_64::_mm256_setr_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_epi32.html)
- [arch::x86_64::_mm256_setr_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_epi64x.html)
- [arch::x86_64::_mm256_setr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_epi8.html)
- [arch::x86_64::_mm256_setr_m128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_m128.html)
- [arch::x86_64::_mm256_setr_m128d](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_m128d.html)
- [arch::x86_64::_mm256_setr_m128i](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_m128i.html)
- [arch::x86_64::_mm256_setr_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_pd.html)
- [arch::x86_64::_mm256_setr_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setr_ps.html)
- [arch::x86_64::_mm256_setzero_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setzero_pd.html)
- [arch::x86_64::_mm256_setzero_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setzero_ps.html)
- [arch::x86_64::_mm256_setzero_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_setzero_si256.html)
- [arch::x86_64::_mm256_shuffle_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_shuffle_epi32.html)
- [arch::x86_64::_mm256_shuffle_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_shuffle_epi8.html)
- [arch::x86_64::_mm256_shuffle_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_shuffle_pd.html)
- [arch::x86_64::_mm256_shuffle_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_shuffle_ps.html)
- [arch::x86_64::_mm256_shufflehi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_shufflehi_epi16.html)
- [arch::x86_64::_mm256_shufflelo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_shufflelo_epi16.html)
- [arch::x86_64::_mm256_sign_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sign_epi16.html)
- [arch::x86_64::_mm256_sign_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sign_epi32.html)
- [arch::x86_64::_mm256_sign_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sign_epi8.html)
- [arch::x86_64::_mm256_sll_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sll_epi16.html)
- [arch::x86_64::_mm256_sll_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sll_epi32.html)
- [arch::x86_64::_mm256_sll_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sll_epi64.html)
- [arch::x86_64::_mm256_slli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_slli_epi16.html)
- [arch::x86_64::_mm256_slli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_slli_epi32.html)
- [arch::x86_64::_mm256_slli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_slli_epi64.html)
- [arch::x86_64::_mm256_slli_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_slli_si256.html)
- [arch::x86_64::_mm256_sllv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sllv_epi32.html)
- [arch::x86_64::_mm256_sllv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sllv_epi64.html)
- [arch::x86_64::_mm256_sqrt_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sqrt_pd.html)
- [arch::x86_64::_mm256_sqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sqrt_ps.html)
- [arch::x86_64::_mm256_sra_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sra_epi16.html)
- [arch::x86_64::_mm256_sra_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sra_epi32.html)
- [arch::x86_64::_mm256_srai_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srai_epi16.html)
- [arch::x86_64::_mm256_srai_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srai_epi32.html)
- [arch::x86_64::_mm256_srav_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srav_epi32.html)
- [arch::x86_64::_mm256_srl_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srl_epi16.html)
- [arch::x86_64::_mm256_srl_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srl_epi32.html)
- [arch::x86_64::_mm256_srl_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srl_epi64.html)
- [arch::x86_64::_mm256_srli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srli_epi16.html)
- [arch::x86_64::_mm256_srli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srli_epi32.html)
- [arch::x86_64::_mm256_srli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srli_epi64.html)
- [arch::x86_64::_mm256_srli_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srli_si256.html)
- [arch::x86_64::_mm256_srlv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srlv_epi32.html)
- [arch::x86_64::_mm256_srlv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_srlv_epi64.html)
- [arch::x86_64::_mm256_store_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_store_pd.html)
- [arch::x86_64::_mm256_store_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_store_ps.html)
- [arch::x86_64::_mm256_store_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_store_si256.html)
- [arch::x86_64::_mm256_storeu2_m128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_storeu2_m128.html)
- [arch::x86_64::_mm256_storeu2_m128d](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_storeu2_m128d.html)
- [arch::x86_64::_mm256_storeu2_m128i](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_storeu2_m128i.html)
- [arch::x86_64::_mm256_storeu_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_storeu_pd.html)
- [arch::x86_64::_mm256_storeu_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_storeu_ps.html)
- [arch::x86_64::_mm256_storeu_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_storeu_si256.html)
- [arch::x86_64::_mm256_stream_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_stream_pd.html)
- [arch::x86_64::_mm256_stream_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_stream_ps.html)
- [arch::x86_64::_mm256_stream_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_stream_si256.html)
- [arch::x86_64::_mm256_sub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sub_epi16.html)
- [arch::x86_64::_mm256_sub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sub_epi32.html)
- [arch::x86_64::_mm256_sub_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sub_epi64.html)
- [arch::x86_64::_mm256_sub_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sub_epi8.html)
- [arch::x86_64::_mm256_sub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sub_pd.html)
- [arch::x86_64::_mm256_sub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_sub_ps.html)
- [arch::x86_64::_mm256_subs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_subs_epi16.html)
- [arch::x86_64::_mm256_subs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_subs_epi8.html)
- [arch::x86_64::_mm256_subs_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_subs_epu16.html)
- [arch::x86_64::_mm256_subs_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_subs_epu8.html)
- [arch::x86_64::_mm256_testc_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testc_pd.html)
- [arch::x86_64::_mm256_testc_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testc_ps.html)
- [arch::x86_64::_mm256_testc_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testc_si256.html)
- [arch::x86_64::_mm256_testnzc_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testnzc_pd.html)
- [arch::x86_64::_mm256_testnzc_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testnzc_ps.html)
- [arch::x86_64::_mm256_testnzc_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testnzc_si256.html)
- [arch::x86_64::_mm256_testz_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testz_pd.html)
- [arch::x86_64::_mm256_testz_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testz_ps.html)
- [arch::x86_64::_mm256_testz_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_testz_si256.html)
- [arch::x86_64::_mm256_undefined_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_undefined_pd.html)
- [arch::x86_64::_mm256_undefined_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_undefined_ps.html)
- [arch::x86_64::_mm256_undefined_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_undefined_si256.html)
- [arch::x86_64::_mm256_unpackhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpackhi_epi16.html)
- [arch::x86_64::_mm256_unpackhi_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpackhi_epi32.html)
- [arch::x86_64::_mm256_unpackhi_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpackhi_epi64.html)
- [arch::x86_64::_mm256_unpackhi_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpackhi_epi8.html)
- [arch::x86_64::_mm256_unpackhi_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpackhi_pd.html)
- [arch::x86_64::_mm256_unpackhi_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpackhi_ps.html)
- [arch::x86_64::_mm256_unpacklo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpacklo_epi16.html)
- [arch::x86_64::_mm256_unpacklo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpacklo_epi32.html)
- [arch::x86_64::_mm256_unpacklo_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpacklo_epi64.html)
- [arch::x86_64::_mm256_unpacklo_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpacklo_epi8.html)
- [arch::x86_64::_mm256_unpacklo_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpacklo_pd.html)
- [arch::x86_64::_mm256_unpacklo_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_unpacklo_ps.html)
- [arch::x86_64::_mm256_xor_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_xor_pd.html)
- [arch::x86_64::_mm256_xor_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_xor_ps.html)
- [arch::x86_64::_mm256_xor_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_xor_si256.html)
- [arch::x86_64::_mm256_zeroall](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_zeroall.html)
- [arch::x86_64::_mm256_zeroupper](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_zeroupper.html)
- [arch::x86_64::_mm256_zextpd128_pd256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_zextpd128_pd256.html)
- [arch::x86_64::_mm256_zextps128_ps256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_zextps128_ps256.html)
- [arch::x86_64::_mm256_zextsi128_si256](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm256_zextsi128_si256.html)
- [arch::x86_64::_mm_abs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_abs_epi16.html)
- [arch::x86_64::_mm_abs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_abs_epi32.html)
- [arch::x86_64::_mm_abs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_abs_epi8.html)
- [arch::x86_64::_mm_abs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_abs_pi16.html)
- [arch::x86_64::_mm_abs_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_abs_pi32.html)
- [arch::x86_64::_mm_abs_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_abs_pi8.html)
- [arch::x86_64::_mm_add_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_epi16.html)
- [arch::x86_64::_mm_add_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_epi32.html)
- [arch::x86_64::_mm_add_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_epi64.html)
- [arch::x86_64::_mm_add_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_epi8.html)
- [arch::x86_64::_mm_add_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_pd.html)
- [arch::x86_64::_mm_add_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_pi16.html)
- [arch::x86_64::_mm_add_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_pi32.html)
- [arch::x86_64::_mm_add_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_pi8.html)
- [arch::x86_64::_mm_add_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_ps.html)
- [arch::x86_64::_mm_add_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_sd.html)
- [arch::x86_64::_mm_add_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_si64.html)
- [arch::x86_64::_mm_add_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_add_ss.html)
- [arch::x86_64::_mm_adds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_epi16.html)
- [arch::x86_64::_mm_adds_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_epi8.html)
- [arch::x86_64::_mm_adds_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_epu16.html)
- [arch::x86_64::_mm_adds_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_epu8.html)
- [arch::x86_64::_mm_adds_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_pi16.html)
- [arch::x86_64::_mm_adds_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_pi8.html)
- [arch::x86_64::_mm_adds_pu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_pu16.html)
- [arch::x86_64::_mm_adds_pu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_adds_pu8.html)
- [arch::x86_64::_mm_addsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_addsub_pd.html)
- [arch::x86_64::_mm_addsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_addsub_ps.html)
- [arch::x86_64::_mm_aesdec_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_aesdec_si128.html)
- [arch::x86_64::_mm_aesdeclast_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_aesdeclast_si128.html)
- [arch::x86_64::_mm_aesenc_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_aesenc_si128.html)
- [arch::x86_64::_mm_aesenclast_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_aesenclast_si128.html)
- [arch::x86_64::_mm_aesimc_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_aesimc_si128.html)
- [arch::x86_64::_mm_aeskeygenassist_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_aeskeygenassist_si128.html)
- [arch::x86_64::_mm_alignr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_alignr_epi8.html)
- [arch::x86_64::_mm_alignr_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_alignr_pi8.html)
- [arch::x86_64::_mm_and_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_and_pd.html)
- [arch::x86_64::_mm_and_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_and_ps.html)
- [arch::x86_64::_mm_and_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_and_si128.html)
- [arch::x86_64::_mm_andnot_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_andnot_pd.html)
- [arch::x86_64::_mm_andnot_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_andnot_ps.html)
- [arch::x86_64::_mm_andnot_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_andnot_si128.html)
- [arch::x86_64::_mm_avg_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_avg_epu16.html)
- [arch::x86_64::_mm_avg_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_avg_epu8.html)
- [arch::x86_64::_mm_avg_pu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_avg_pu16.html)
- [arch::x86_64::_mm_avg_pu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_avg_pu8.html)
- [arch::x86_64::_mm_blend_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blend_epi16.html)
- [arch::x86_64::_mm_blend_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blend_epi32.html)
- [arch::x86_64::_mm_blend_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blend_pd.html)
- [arch::x86_64::_mm_blend_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blend_ps.html)
- [arch::x86_64::_mm_blendv_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blendv_epi8.html)
- [arch::x86_64::_mm_blendv_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blendv_pd.html)
- [arch::x86_64::_mm_blendv_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_blendv_ps.html)
- [arch::x86_64::_mm_broadcast_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcast_ss.html)
- [arch::x86_64::_mm_broadcastb_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcastb_epi8.html)
- [arch::x86_64::_mm_broadcastd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcastd_epi32.html)
- [arch::x86_64::_mm_broadcastq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcastq_epi64.html)
- [arch::x86_64::_mm_broadcastsd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcastsd_pd.html)
- [arch::x86_64::_mm_broadcastss_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcastss_ps.html)
- [arch::x86_64::_mm_broadcastw_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_broadcastw_epi16.html)
- [arch::x86_64::_mm_bslli_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_bslli_si128.html)
- [arch::x86_64::_mm_bsrli_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_bsrli_si128.html)
- [arch::x86_64::_mm_castpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_castpd_ps.html)
- [arch::x86_64::_mm_castpd_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_castpd_si128.html)
- [arch::x86_64::_mm_castps_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_castps_pd.html)
- [arch::x86_64::_mm_castps_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_castps_si128.html)
- [arch::x86_64::_mm_castsi128_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_castsi128_pd.html)
- [arch::x86_64::_mm_castsi128_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_castsi128_ps.html)
- [arch::x86_64::_mm_ceil_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ceil_pd.html)
- [arch::x86_64::_mm_ceil_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ceil_ps.html)
- [arch::x86_64::_mm_ceil_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ceil_sd.html)
- [arch::x86_64::_mm_ceil_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ceil_ss.html)
- [arch::x86_64::_mm_clflush](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_clflush.html)
- [arch::x86_64::_mm_clmulepi64_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_clmulepi64_si128.html)
- [arch::x86_64::_mm_cmp_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmp_pd.html)
- [arch::x86_64::_mm_cmp_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmp_ps.html)
- [arch::x86_64::_mm_cmp_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmp_sd.html)
- [arch::x86_64::_mm_cmp_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmp_ss.html)
- [arch::x86_64::_mm_cmpeq_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_epi16.html)
- [arch::x86_64::_mm_cmpeq_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_epi32.html)
- [arch::x86_64::_mm_cmpeq_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_epi64.html)
- [arch::x86_64::_mm_cmpeq_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_epi8.html)
- [arch::x86_64::_mm_cmpeq_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_pd.html)
- [arch::x86_64::_mm_cmpeq_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_ps.html)
- [arch::x86_64::_mm_cmpeq_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_sd.html)
- [arch::x86_64::_mm_cmpeq_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpeq_ss.html)
- [arch::x86_64::_mm_cmpestra](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestra.html)
- [arch::x86_64::_mm_cmpestrc](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestrc.html)
- [arch::x86_64::_mm_cmpestri](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestri.html)
- [arch::x86_64::_mm_cmpestrm](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestrm.html)
- [arch::x86_64::_mm_cmpestro](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestro.html)
- [arch::x86_64::_mm_cmpestrs](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestrs.html)
- [arch::x86_64::_mm_cmpestrz](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpestrz.html)
- [arch::x86_64::_mm_cmpge_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpge_pd.html)
- [arch::x86_64::_mm_cmpge_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpge_ps.html)
- [arch::x86_64::_mm_cmpge_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpge_sd.html)
- [arch::x86_64::_mm_cmpge_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpge_ss.html)
- [arch::x86_64::_mm_cmpgt_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_epi16.html)
- [arch::x86_64::_mm_cmpgt_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_epi32.html)
- [arch::x86_64::_mm_cmpgt_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_epi64.html)
- [arch::x86_64::_mm_cmpgt_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_epi8.html)
- [arch::x86_64::_mm_cmpgt_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_pd.html)
- [arch::x86_64::_mm_cmpgt_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_pi16.html)
- [arch::x86_64::_mm_cmpgt_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_pi32.html)
- [arch::x86_64::_mm_cmpgt_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_pi8.html)
- [arch::x86_64::_mm_cmpgt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_ps.html)
- [arch::x86_64::_mm_cmpgt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_sd.html)
- [arch::x86_64::_mm_cmpgt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpgt_ss.html)
- [arch::x86_64::_mm_cmpistra](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistra.html)
- [arch::x86_64::_mm_cmpistrc](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistrc.html)
- [arch::x86_64::_mm_cmpistri](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistri.html)
- [arch::x86_64::_mm_cmpistrm](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistrm.html)
- [arch::x86_64::_mm_cmpistro](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistro.html)
- [arch::x86_64::_mm_cmpistrs](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistrs.html)
- [arch::x86_64::_mm_cmpistrz](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpistrz.html)
- [arch::x86_64::_mm_cmple_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmple_pd.html)
- [arch::x86_64::_mm_cmple_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmple_ps.html)
- [arch::x86_64::_mm_cmple_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmple_sd.html)
- [arch::x86_64::_mm_cmple_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmple_ss.html)
- [arch::x86_64::_mm_cmplt_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_epi16.html)
- [arch::x86_64::_mm_cmplt_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_epi32.html)
- [arch::x86_64::_mm_cmplt_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_epi8.html)
- [arch::x86_64::_mm_cmplt_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_pd.html)
- [arch::x86_64::_mm_cmplt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_ps.html)
- [arch::x86_64::_mm_cmplt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_sd.html)
- [arch::x86_64::_mm_cmplt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmplt_ss.html)
- [arch::x86_64::_mm_cmpneq_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpneq_pd.html)
- [arch::x86_64::_mm_cmpneq_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpneq_ps.html)
- [arch::x86_64::_mm_cmpneq_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpneq_sd.html)
- [arch::x86_64::_mm_cmpneq_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpneq_ss.html)
- [arch::x86_64::_mm_cmpnge_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnge_pd.html)
- [arch::x86_64::_mm_cmpnge_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnge_ps.html)
- [arch::x86_64::_mm_cmpnge_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnge_sd.html)
- [arch::x86_64::_mm_cmpnge_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnge_ss.html)
- [arch::x86_64::_mm_cmpngt_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpngt_pd.html)
- [arch::x86_64::_mm_cmpngt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpngt_ps.html)
- [arch::x86_64::_mm_cmpngt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpngt_sd.html)
- [arch::x86_64::_mm_cmpngt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpngt_ss.html)
- [arch::x86_64::_mm_cmpnle_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnle_pd.html)
- [arch::x86_64::_mm_cmpnle_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnle_ps.html)
- [arch::x86_64::_mm_cmpnle_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnle_sd.html)
- [arch::x86_64::_mm_cmpnle_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnle_ss.html)
- [arch::x86_64::_mm_cmpnlt_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnlt_pd.html)
- [arch::x86_64::_mm_cmpnlt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnlt_ps.html)
- [arch::x86_64::_mm_cmpnlt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnlt_sd.html)
- [arch::x86_64::_mm_cmpnlt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpnlt_ss.html)
- [arch::x86_64::_mm_cmpord_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpord_pd.html)
- [arch::x86_64::_mm_cmpord_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpord_ps.html)
- [arch::x86_64::_mm_cmpord_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpord_sd.html)
- [arch::x86_64::_mm_cmpord_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpord_ss.html)
- [arch::x86_64::_mm_cmpunord_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpunord_pd.html)
- [arch::x86_64::_mm_cmpunord_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpunord_ps.html)
- [arch::x86_64::_mm_cmpunord_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpunord_sd.html)
- [arch::x86_64::_mm_cmpunord_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cmpunord_ss.html)
- [arch::x86_64::_mm_comieq_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comieq_sd.html)
- [arch::x86_64::_mm_comieq_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comieq_ss.html)
- [arch::x86_64::_mm_comige_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comige_sd.html)
- [arch::x86_64::_mm_comige_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comige_ss.html)
- [arch::x86_64::_mm_comigt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comigt_sd.html)
- [arch::x86_64::_mm_comigt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comigt_ss.html)
- [arch::x86_64::_mm_comile_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comile_sd.html)
- [arch::x86_64::_mm_comile_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comile_ss.html)
- [arch::x86_64::_mm_comilt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comilt_sd.html)
- [arch::x86_64::_mm_comilt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comilt_ss.html)
- [arch::x86_64::_mm_comineq_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comineq_sd.html)
- [arch::x86_64::_mm_comineq_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_comineq_ss.html)
- [arch::x86_64::_mm_crc32_u16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_crc32_u16.html)
- [arch::x86_64::_mm_crc32_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_crc32_u32.html)
- [arch::x86_64::_mm_crc32_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_crc32_u64.html)
- [arch::x86_64::_mm_crc32_u8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_crc32_u8.html)
- [arch::x86_64::_mm_cvt_pi2ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvt_pi2ps.html)
- [arch::x86_64::_mm_cvt_ps2pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvt_ps2pi.html)
- [arch::x86_64::_mm_cvt_si2ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvt_si2ss.html)
- [arch::x86_64::_mm_cvt_ss2si](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvt_ss2si.html)
- [arch::x86_64::_mm_cvtepi16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi16_epi32.html)
- [arch::x86_64::_mm_cvtepi16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi16_epi64.html)
- [arch::x86_64::_mm_cvtepi32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi32_epi64.html)
- [arch::x86_64::_mm_cvtepi32_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi32_pd.html)
- [arch::x86_64::_mm_cvtepi32_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi32_ps.html)
- [arch::x86_64::_mm_cvtepi8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi8_epi16.html)
- [arch::x86_64::_mm_cvtepi8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi8_epi32.html)
- [arch::x86_64::_mm_cvtepi8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepi8_epi64.html)
- [arch::x86_64::_mm_cvtepu16_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepu16_epi32.html)
- [arch::x86_64::_mm_cvtepu16_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepu16_epi64.html)
- [arch::x86_64::_mm_cvtepu32_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepu32_epi64.html)
- [arch::x86_64::_mm_cvtepu8_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepu8_epi16.html)
- [arch::x86_64::_mm_cvtepu8_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepu8_epi32.html)
- [arch::x86_64::_mm_cvtepu8_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtepu8_epi64.html)
- [arch::x86_64::_mm_cvtpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpd_epi32.html)
- [arch::x86_64::_mm_cvtpd_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpd_pi32.html)
- [arch::x86_64::_mm_cvtpd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpd_ps.html)
- [arch::x86_64::_mm_cvtpi16_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpi16_ps.html)
- [arch::x86_64::_mm_cvtpi32_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpi32_pd.html)
- [arch::x86_64::_mm_cvtpi32_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpi32_ps.html)
- [arch::x86_64::_mm_cvtpi32x2_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpi32x2_ps.html)
- [arch::x86_64::_mm_cvtpi8_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpi8_ps.html)
- [arch::x86_64::_mm_cvtps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtps_epi32.html)
- [arch::x86_64::_mm_cvtps_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtps_pd.html)
- [arch::x86_64::_mm_cvtps_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtps_pi16.html)
- [arch::x86_64::_mm_cvtps_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtps_pi32.html)
- [arch::x86_64::_mm_cvtps_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtps_pi8.html)
- [arch::x86_64::_mm_cvtpu16_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpu16_ps.html)
- [arch::x86_64::_mm_cvtpu8_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtpu8_ps.html)
- [arch::x86_64::_mm_cvtsd_f64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsd_f64.html)
- [arch::x86_64::_mm_cvtsd_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsd_si32.html)
- [arch::x86_64::_mm_cvtsd_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsd_si64.html)
- [arch::x86_64::_mm_cvtsd_si64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsd_si64x.html)
- [arch::x86_64::_mm_cvtsd_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsd_ss.html)
- [arch::x86_64::_mm_cvtsi128_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi128_si32.html)
- [arch::x86_64::_mm_cvtsi128_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi128_si64.html)
- [arch::x86_64::_mm_cvtsi128_si64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi128_si64x.html)
- [arch::x86_64::_mm_cvtsi32_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi32_sd.html)
- [arch::x86_64::_mm_cvtsi32_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi32_si128.html)
- [arch::x86_64::_mm_cvtsi32_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi32_ss.html)
- [arch::x86_64::_mm_cvtsi64_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi64_sd.html)
- [arch::x86_64::_mm_cvtsi64_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi64_si128.html)
- [arch::x86_64::_mm_cvtsi64_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi64_ss.html)
- [arch::x86_64::_mm_cvtsi64x_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi64x_sd.html)
- [arch::x86_64::_mm_cvtsi64x_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtsi64x_si128.html)
- [arch::x86_64::_mm_cvtss_f32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtss_f32.html)
- [arch::x86_64::_mm_cvtss_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtss_sd.html)
- [arch::x86_64::_mm_cvtss_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtss_si32.html)
- [arch::x86_64::_mm_cvtss_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtss_si64.html)
- [arch::x86_64::_mm_cvtt_ps2pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtt_ps2pi.html)
- [arch::x86_64::_mm_cvtt_ss2si](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvtt_ss2si.html)
- [arch::x86_64::_mm_cvttpd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttpd_epi32.html)
- [arch::x86_64::_mm_cvttpd_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttpd_pi32.html)
- [arch::x86_64::_mm_cvttps_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttps_epi32.html)
- [arch::x86_64::_mm_cvttps_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttps_pi32.html)
- [arch::x86_64::_mm_cvttsd_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttsd_si32.html)
- [arch::x86_64::_mm_cvttsd_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttsd_si64.html)
- [arch::x86_64::_mm_cvttsd_si64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttsd_si64x.html)
- [arch::x86_64::_mm_cvttss_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttss_si32.html)
- [arch::x86_64::_mm_cvttss_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_cvttss_si64.html)
- [arch::x86_64::_mm_div_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_div_pd.html)
- [arch::x86_64::_mm_div_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_div_ps.html)
- [arch::x86_64::_mm_div_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_div_sd.html)
- [arch::x86_64::_mm_div_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_div_ss.html)
- [arch::x86_64::_mm_dp_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_dp_pd.html)
- [arch::x86_64::_mm_dp_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_dp_ps.html)
- [arch::x86_64::_mm_extract_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_epi16.html)
- [arch::x86_64::_mm_extract_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_epi32.html)
- [arch::x86_64::_mm_extract_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_epi64.html)
- [arch::x86_64::_mm_extract_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_epi8.html)
- [arch::x86_64::_mm_extract_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_pi16.html)
- [arch::x86_64::_mm_extract_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_ps.html)
- [arch::x86_64::_mm_extract_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_extract_si64.html)
- [arch::x86_64::_mm_floor_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_floor_pd.html)
- [arch::x86_64::_mm_floor_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_floor_ps.html)
- [arch::x86_64::_mm_floor_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_floor_sd.html)
- [arch::x86_64::_mm_floor_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_floor_ss.html)
- [arch::x86_64::_mm_fmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmadd_pd.html)
- [arch::x86_64::_mm_fmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmadd_ps.html)
- [arch::x86_64::_mm_fmadd_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmadd_sd.html)
- [arch::x86_64::_mm_fmadd_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmadd_ss.html)
- [arch::x86_64::_mm_fmaddsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmaddsub_pd.html)
- [arch::x86_64::_mm_fmaddsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmaddsub_ps.html)
- [arch::x86_64::_mm_fmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmsub_pd.html)
- [arch::x86_64::_mm_fmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmsub_ps.html)
- [arch::x86_64::_mm_fmsub_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmsub_sd.html)
- [arch::x86_64::_mm_fmsub_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmsub_ss.html)
- [arch::x86_64::_mm_fmsubadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmsubadd_pd.html)
- [arch::x86_64::_mm_fmsubadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fmsubadd_ps.html)
- [arch::x86_64::_mm_fnmadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmadd_pd.html)
- [arch::x86_64::_mm_fnmadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmadd_ps.html)
- [arch::x86_64::_mm_fnmadd_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmadd_sd.html)
- [arch::x86_64::_mm_fnmadd_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmadd_ss.html)
- [arch::x86_64::_mm_fnmsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmsub_pd.html)
- [arch::x86_64::_mm_fnmsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmsub_ps.html)
- [arch::x86_64::_mm_fnmsub_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmsub_sd.html)
- [arch::x86_64::_mm_fnmsub_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_fnmsub_ss.html)
- [arch::x86_64::_mm_getcsr](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_getcsr.html)
- [arch::x86_64::_mm_hadd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadd_epi16.html)
- [arch::x86_64::_mm_hadd_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadd_epi32.html)
- [arch::x86_64::_mm_hadd_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadd_pd.html)
- [arch::x86_64::_mm_hadd_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadd_pi16.html)
- [arch::x86_64::_mm_hadd_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadd_pi32.html)
- [arch::x86_64::_mm_hadd_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadd_ps.html)
- [arch::x86_64::_mm_hadds_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadds_epi16.html)
- [arch::x86_64::_mm_hadds_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hadds_pi16.html)
- [arch::x86_64::_mm_hsub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsub_epi16.html)
- [arch::x86_64::_mm_hsub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsub_epi32.html)
- [arch::x86_64::_mm_hsub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsub_pd.html)
- [arch::x86_64::_mm_hsub_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsub_pi16.html)
- [arch::x86_64::_mm_hsub_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsub_pi32.html)
- [arch::x86_64::_mm_hsub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsub_ps.html)
- [arch::x86_64::_mm_hsubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsubs_epi16.html)
- [arch::x86_64::_mm_hsubs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_hsubs_pi16.html)
- [arch::x86_64::_mm_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i32gather_epi32.html)
- [arch::x86_64::_mm_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i32gather_epi64.html)
- [arch::x86_64::_mm_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i32gather_pd.html)
- [arch::x86_64::_mm_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i32gather_ps.html)
- [arch::x86_64::_mm_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i64gather_epi32.html)
- [arch::x86_64::_mm_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i64gather_epi64.html)
- [arch::x86_64::_mm_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i64gather_pd.html)
- [arch::x86_64::_mm_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_i64gather_ps.html)
- [arch::x86_64::_mm_insert_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_epi16.html)
- [arch::x86_64::_mm_insert_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_epi32.html)
- [arch::x86_64::_mm_insert_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_epi64.html)
- [arch::x86_64::_mm_insert_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_epi8.html)
- [arch::x86_64::_mm_insert_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_pi16.html)
- [arch::x86_64::_mm_insert_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_ps.html)
- [arch::x86_64::_mm_insert_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_insert_si64.html)
- [arch::x86_64::_mm_lddqu_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_lddqu_si128.html)
- [arch::x86_64::_mm_lfence](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_lfence.html)
- [arch::x86_64::_mm_load1_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load1_pd.html)
- [arch::x86_64::_mm_load1_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load1_ps.html)
- [arch::x86_64::_mm_load_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_pd.html)
- [arch::x86_64::_mm_load_pd1](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_pd1.html)
- [arch::x86_64::_mm_load_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_ps.html)
- [arch::x86_64::_mm_load_ps1](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_ps1.html)
- [arch::x86_64::_mm_load_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_sd.html)
- [arch::x86_64::_mm_load_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_si128.html)
- [arch::x86_64::_mm_load_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_load_ss.html)
- [arch::x86_64::_mm_loaddup_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loaddup_pd.html)
- [arch::x86_64::_mm_loadh_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadh_pd.html)
- [arch::x86_64::_mm_loadh_pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadh_pi.html)
- [arch::x86_64::_mm_loadl_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadl_epi64.html)
- [arch::x86_64::_mm_loadl_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadl_pd.html)
- [arch::x86_64::_mm_loadl_pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadl_pi.html)
- [arch::x86_64::_mm_loadr_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadr_pd.html)
- [arch::x86_64::_mm_loadr_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadr_ps.html)
- [arch::x86_64::_mm_loadu_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadu_pd.html)
- [arch::x86_64::_mm_loadu_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadu_ps.html)
- [arch::x86_64::_mm_loadu_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_loadu_si128.html)
- [arch::x86_64::_mm_madd_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_madd_epi16.html)
- [arch::x86_64::_mm_maddubs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maddubs_epi16.html)
- [arch::x86_64::_mm_maddubs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maddubs_pi16.html)
- [arch::x86_64::_mm_mask_i32gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i32gather_epi32.html)
- [arch::x86_64::_mm_mask_i32gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i32gather_epi64.html)
- [arch::x86_64::_mm_mask_i32gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i32gather_pd.html)
- [arch::x86_64::_mm_mask_i32gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i32gather_ps.html)
- [arch::x86_64::_mm_mask_i64gather_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i64gather_epi32.html)
- [arch::x86_64::_mm_mask_i64gather_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i64gather_epi64.html)
- [arch::x86_64::_mm_mask_i64gather_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i64gather_pd.html)
- [arch::x86_64::_mm_mask_i64gather_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mask_i64gather_ps.html)
- [arch::x86_64::_mm_maskload_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskload_epi32.html)
- [arch::x86_64::_mm_maskload_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskload_epi64.html)
- [arch::x86_64::_mm_maskload_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskload_pd.html)
- [arch::x86_64::_mm_maskload_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskload_ps.html)
- [arch::x86_64::_mm_maskmove_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskmove_si64.html)
- [arch::x86_64::_mm_maskmoveu_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskmoveu_si128.html)
- [arch::x86_64::_mm_maskstore_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskstore_epi32.html)
- [arch::x86_64::_mm_maskstore_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskstore_epi64.html)
- [arch::x86_64::_mm_maskstore_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskstore_pd.html)
- [arch::x86_64::_mm_maskstore_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_maskstore_ps.html)
- [arch::x86_64::_mm_max_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_epi16.html)
- [arch::x86_64::_mm_max_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_epi32.html)
- [arch::x86_64::_mm_max_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_epi8.html)
- [arch::x86_64::_mm_max_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_epu16.html)
- [arch::x86_64::_mm_max_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_epu32.html)
- [arch::x86_64::_mm_max_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_epu8.html)
- [arch::x86_64::_mm_max_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_pd.html)
- [arch::x86_64::_mm_max_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_pi16.html)
- [arch::x86_64::_mm_max_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_ps.html)
- [arch::x86_64::_mm_max_pu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_pu8.html)
- [arch::x86_64::_mm_max_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_sd.html)
- [arch::x86_64::_mm_max_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_max_ss.html)
- [arch::x86_64::_mm_mfence](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mfence.html)
- [arch::x86_64::_mm_min_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_epi16.html)
- [arch::x86_64::_mm_min_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_epi32.html)
- [arch::x86_64::_mm_min_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_epi8.html)
- [arch::x86_64::_mm_min_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_epu16.html)
- [arch::x86_64::_mm_min_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_epu32.html)
- [arch::x86_64::_mm_min_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_epu8.html)
- [arch::x86_64::_mm_min_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_pd.html)
- [arch::x86_64::_mm_min_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_pi16.html)
- [arch::x86_64::_mm_min_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_ps.html)
- [arch::x86_64::_mm_min_pu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_pu8.html)
- [arch::x86_64::_mm_min_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_sd.html)
- [arch::x86_64::_mm_min_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_min_ss.html)
- [arch::x86_64::_mm_minpos_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_minpos_epu16.html)
- [arch::x86_64::_mm_move_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_move_epi64.html)
- [arch::x86_64::_mm_move_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_move_sd.html)
- [arch::x86_64::_mm_move_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_move_ss.html)
- [arch::x86_64::_mm_movedup_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movedup_pd.html)
- [arch::x86_64::_mm_movehdup_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movehdup_ps.html)
- [arch::x86_64::_mm_movehl_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movehl_ps.html)
- [arch::x86_64::_mm_moveldup_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_moveldup_ps.html)
- [arch::x86_64::_mm_movelh_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movelh_ps.html)
- [arch::x86_64::_mm_movemask_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movemask_epi8.html)
- [arch::x86_64::_mm_movemask_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movemask_pd.html)
- [arch::x86_64::_mm_movemask_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movemask_pi8.html)
- [arch::x86_64::_mm_movemask_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movemask_ps.html)
- [arch::x86_64::_mm_movepi64_pi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movepi64_pi64.html)
- [arch::x86_64::_mm_movpi64_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_movpi64_epi64.html)
- [arch::x86_64::_mm_mpsadbw_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mpsadbw_epu8.html)
- [arch::x86_64::_mm_mul_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_epi32.html)
- [arch::x86_64::_mm_mul_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_epu32.html)
- [arch::x86_64::_mm_mul_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_pd.html)
- [arch::x86_64::_mm_mul_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_ps.html)
- [arch::x86_64::_mm_mul_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_sd.html)
- [arch::x86_64::_mm_mul_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_ss.html)
- [arch::x86_64::_mm_mul_su32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mul_su32.html)
- [arch::x86_64::_mm_mulhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mulhi_epi16.html)
- [arch::x86_64::_mm_mulhi_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mulhi_epu16.html)
- [arch::x86_64::_mm_mulhi_pu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mulhi_pu16.html)
- [arch::x86_64::_mm_mulhrs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mulhrs_epi16.html)
- [arch::x86_64::_mm_mulhrs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mulhrs_pi16.html)
- [arch::x86_64::_mm_mullo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mullo_epi16.html)
- [arch::x86_64::_mm_mullo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_mullo_epi32.html)
- [arch::x86_64::_mm_or_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_or_pd.html)
- [arch::x86_64::_mm_or_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_or_ps.html)
- [arch::x86_64::_mm_or_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_or_si128.html)
- [arch::x86_64::_mm_packs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_packs_epi16.html)
- [arch::x86_64::_mm_packs_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_packs_epi32.html)
- [arch::x86_64::_mm_packs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_packs_pi16.html)
- [arch::x86_64::_mm_packs_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_packs_pi32.html)
- [arch::x86_64::_mm_packus_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_packus_epi16.html)
- [arch::x86_64::_mm_packus_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_packus_epi32.html)
- [arch::x86_64::_mm_pause](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_pause.html)
- [arch::x86_64::_mm_permute_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_permute_pd.html)
- [arch::x86_64::_mm_permute_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_permute_ps.html)
- [arch::x86_64::_mm_permutevar_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_permutevar_pd.html)
- [arch::x86_64::_mm_permutevar_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_permutevar_ps.html)
- [arch::x86_64::_mm_prefetch](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_prefetch.html)
- [arch::x86_64::_mm_rcp_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_rcp_ps.html)
- [arch::x86_64::_mm_rcp_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_rcp_ss.html)
- [arch::x86_64::_mm_round_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_round_pd.html)
- [arch::x86_64::_mm_round_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_round_ps.html)
- [arch::x86_64::_mm_round_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_round_sd.html)
- [arch::x86_64::_mm_round_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_round_ss.html)
- [arch::x86_64::_mm_rsqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_rsqrt_ps.html)
- [arch::x86_64::_mm_rsqrt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_rsqrt_ss.html)
- [arch::x86_64::_mm_sad_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sad_epu8.html)
- [arch::x86_64::_mm_sad_pu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sad_pu8.html)
- [arch::x86_64::_mm_set1_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_epi16.html)
- [arch::x86_64::_mm_set1_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_epi32.html)
- [arch::x86_64::_mm_set1_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_epi64.html)
- [arch::x86_64::_mm_set1_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_epi64x.html)
- [arch::x86_64::_mm_set1_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_epi8.html)
- [arch::x86_64::_mm_set1_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_pd.html)
- [arch::x86_64::_mm_set1_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_pi16.html)
- [arch::x86_64::_mm_set1_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_pi32.html)
- [arch::x86_64::_mm_set1_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_pi8.html)
- [arch::x86_64::_mm_set1_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set1_ps.html)
- [arch::x86_64::_mm_set_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_epi16.html)
- [arch::x86_64::_mm_set_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_epi32.html)
- [arch::x86_64::_mm_set_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_epi64.html)
- [arch::x86_64::_mm_set_epi64x](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_epi64x.html)
- [arch::x86_64::_mm_set_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_epi8.html)
- [arch::x86_64::_mm_set_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_pd.html)
- [arch::x86_64::_mm_set_pd1](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_pd1.html)
- [arch::x86_64::_mm_set_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_pi16.html)
- [arch::x86_64::_mm_set_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_pi32.html)
- [arch::x86_64::_mm_set_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_pi8.html)
- [arch::x86_64::_mm_set_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_ps.html)
- [arch::x86_64::_mm_set_ps1](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_ps1.html)
- [arch::x86_64::_mm_set_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_sd.html)
- [arch::x86_64::_mm_set_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_set_ss.html)
- [arch::x86_64::_mm_setcsr](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setcsr.html)
- [arch::x86_64::_mm_setr_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_epi16.html)
- [arch::x86_64::_mm_setr_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_epi32.html)
- [arch::x86_64::_mm_setr_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_epi64.html)
- [arch::x86_64::_mm_setr_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_epi8.html)
- [arch::x86_64::_mm_setr_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_pd.html)
- [arch::x86_64::_mm_setr_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_pi16.html)
- [arch::x86_64::_mm_setr_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_pi32.html)
- [arch::x86_64::_mm_setr_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_pi8.html)
- [arch::x86_64::_mm_setr_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setr_ps.html)
- [arch::x86_64::_mm_setzero_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setzero_pd.html)
- [arch::x86_64::_mm_setzero_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setzero_ps.html)
- [arch::x86_64::_mm_setzero_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setzero_si128.html)
- [arch::x86_64::_mm_setzero_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_setzero_si64.html)
- [arch::x86_64::_mm_sfence](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sfence.html)
- [arch::x86_64::_mm_sha1msg1_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha1msg1_epu32.html)
- [arch::x86_64::_mm_sha1msg2_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha1msg2_epu32.html)
- [arch::x86_64::_mm_sha1nexte_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha1nexte_epu32.html)
- [arch::x86_64::_mm_sha1rnds4_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha1rnds4_epu32.html)
- [arch::x86_64::_mm_sha256msg1_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha256msg1_epu32.html)
- [arch::x86_64::_mm_sha256msg2_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha256msg2_epu32.html)
- [arch::x86_64::_mm_sha256rnds2_epu32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sha256rnds2_epu32.html)
- [arch::x86_64::_mm_shuffle_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shuffle_epi32.html)
- [arch::x86_64::_mm_shuffle_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shuffle_epi8.html)
- [arch::x86_64::_mm_shuffle_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shuffle_pd.html)
- [arch::x86_64::_mm_shuffle_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shuffle_pi16.html)
- [arch::x86_64::_mm_shuffle_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shuffle_pi8.html)
- [arch::x86_64::_mm_shuffle_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shuffle_ps.html)
- [arch::x86_64::_mm_shufflehi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shufflehi_epi16.html)
- [arch::x86_64::_mm_shufflelo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_shufflelo_epi16.html)
- [arch::x86_64::_mm_sign_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sign_epi16.html)
- [arch::x86_64::_mm_sign_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sign_epi32.html)
- [arch::x86_64::_mm_sign_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sign_epi8.html)
- [arch::x86_64::_mm_sign_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sign_pi16.html)
- [arch::x86_64::_mm_sign_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sign_pi32.html)
- [arch::x86_64::_mm_sign_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sign_pi8.html)
- [arch::x86_64::_mm_sll_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sll_epi16.html)
- [arch::x86_64::_mm_sll_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sll_epi32.html)
- [arch::x86_64::_mm_sll_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sll_epi64.html)
- [arch::x86_64::_mm_slli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_slli_epi16.html)
- [arch::x86_64::_mm_slli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_slli_epi32.html)
- [arch::x86_64::_mm_slli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_slli_epi64.html)
- [arch::x86_64::_mm_slli_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_slli_si128.html)
- [arch::x86_64::_mm_sllv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sllv_epi32.html)
- [arch::x86_64::_mm_sllv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sllv_epi64.html)
- [arch::x86_64::_mm_sqrt_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sqrt_pd.html)
- [arch::x86_64::_mm_sqrt_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sqrt_ps.html)
- [arch::x86_64::_mm_sqrt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sqrt_sd.html)
- [arch::x86_64::_mm_sqrt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sqrt_ss.html)
- [arch::x86_64::_mm_sra_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sra_epi16.html)
- [arch::x86_64::_mm_sra_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sra_epi32.html)
- [arch::x86_64::_mm_srai_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srai_epi16.html)
- [arch::x86_64::_mm_srai_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srai_epi32.html)
- [arch::x86_64::_mm_srav_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srav_epi32.html)
- [arch::x86_64::_mm_srl_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srl_epi16.html)
- [arch::x86_64::_mm_srl_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srl_epi32.html)
- [arch::x86_64::_mm_srl_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srl_epi64.html)
- [arch::x86_64::_mm_srli_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srli_epi16.html)
- [arch::x86_64::_mm_srli_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srli_epi32.html)
- [arch::x86_64::_mm_srli_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srli_epi64.html)
- [arch::x86_64::_mm_srli_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srli_si128.html)
- [arch::x86_64::_mm_srlv_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srlv_epi32.html)
- [arch::x86_64::_mm_srlv_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_srlv_epi64.html)
- [arch::x86_64::_mm_store1_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store1_pd.html)
- [arch::x86_64::_mm_store1_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store1_ps.html)
- [arch::x86_64::_mm_store_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_pd.html)
- [arch::x86_64::_mm_store_pd1](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_pd1.html)
- [arch::x86_64::_mm_store_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_ps.html)
- [arch::x86_64::_mm_store_ps1](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_ps1.html)
- [arch::x86_64::_mm_store_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_sd.html)
- [arch::x86_64::_mm_store_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_si128.html)
- [arch::x86_64::_mm_store_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_store_ss.html)
- [arch::x86_64::_mm_storeh_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storeh_pd.html)
- [arch::x86_64::_mm_storeh_pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storeh_pi.html)
- [arch::x86_64::_mm_storel_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storel_epi64.html)
- [arch::x86_64::_mm_storel_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storel_pd.html)
- [arch::x86_64::_mm_storel_pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storel_pi.html)
- [arch::x86_64::_mm_storer_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storer_pd.html)
- [arch::x86_64::_mm_storer_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storer_ps.html)
- [arch::x86_64::_mm_storeu_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storeu_pd.html)
- [arch::x86_64::_mm_storeu_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storeu_ps.html)
- [arch::x86_64::_mm_storeu_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_storeu_si128.html)
- [arch::x86_64::_mm_stream_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_pd.html)
- [arch::x86_64::_mm_stream_pi](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_pi.html)
- [arch::x86_64::_mm_stream_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_ps.html)
- [arch::x86_64::_mm_stream_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_sd.html)
- [arch::x86_64::_mm_stream_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_si128.html)
- [arch::x86_64::_mm_stream_si32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_si32.html)
- [arch::x86_64::_mm_stream_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_si64.html)
- [arch::x86_64::_mm_stream_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_stream_ss.html)
- [arch::x86_64::_mm_sub_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_epi16.html)
- [arch::x86_64::_mm_sub_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_epi32.html)
- [arch::x86_64::_mm_sub_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_epi64.html)
- [arch::x86_64::_mm_sub_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_epi8.html)
- [arch::x86_64::_mm_sub_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_pd.html)
- [arch::x86_64::_mm_sub_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_pi16.html)
- [arch::x86_64::_mm_sub_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_pi32.html)
- [arch::x86_64::_mm_sub_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_pi8.html)
- [arch::x86_64::_mm_sub_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_ps.html)
- [arch::x86_64::_mm_sub_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_sd.html)
- [arch::x86_64::_mm_sub_si64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_si64.html)
- [arch::x86_64::_mm_sub_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_sub_ss.html)
- [arch::x86_64::_mm_subs_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_epi16.html)
- [arch::x86_64::_mm_subs_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_epi8.html)
- [arch::x86_64::_mm_subs_epu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_epu16.html)
- [arch::x86_64::_mm_subs_epu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_epu8.html)
- [arch::x86_64::_mm_subs_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_pi16.html)
- [arch::x86_64::_mm_subs_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_pi8.html)
- [arch::x86_64::_mm_subs_pu16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_pu16.html)
- [arch::x86_64::_mm_subs_pu8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_subs_pu8.html)
- [arch::x86_64::_mm_test_all_ones](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_test_all_ones.html)
- [arch::x86_64::_mm_test_all_zeros](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_test_all_zeros.html)
- [arch::x86_64::_mm_test_mix_ones_zeros](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_test_mix_ones_zeros.html)
- [arch::x86_64::_mm_testc_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testc_pd.html)
- [arch::x86_64::_mm_testc_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testc_ps.html)
- [arch::x86_64::_mm_testc_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testc_si128.html)
- [arch::x86_64::_mm_testnzc_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testnzc_pd.html)
- [arch::x86_64::_mm_testnzc_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testnzc_ps.html)
- [arch::x86_64::_mm_testnzc_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testnzc_si128.html)
- [arch::x86_64::_mm_testz_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testz_pd.html)
- [arch::x86_64::_mm_testz_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testz_ps.html)
- [arch::x86_64::_mm_testz_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_testz_si128.html)
- [arch::x86_64::_mm_tzcnt_32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_tzcnt_32.html)
- [arch::x86_64::_mm_tzcnt_64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_tzcnt_64.html)
- [arch::x86_64::_mm_ucomieq_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomieq_sd.html)
- [arch::x86_64::_mm_ucomieq_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomieq_ss.html)
- [arch::x86_64::_mm_ucomige_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomige_sd.html)
- [arch::x86_64::_mm_ucomige_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomige_ss.html)
- [arch::x86_64::_mm_ucomigt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomigt_sd.html)
- [arch::x86_64::_mm_ucomigt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomigt_ss.html)
- [arch::x86_64::_mm_ucomile_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomile_sd.html)
- [arch::x86_64::_mm_ucomile_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomile_ss.html)
- [arch::x86_64::_mm_ucomilt_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomilt_sd.html)
- [arch::x86_64::_mm_ucomilt_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomilt_ss.html)
- [arch::x86_64::_mm_ucomineq_sd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomineq_sd.html)
- [arch::x86_64::_mm_ucomineq_ss](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_ucomineq_ss.html)
- [arch::x86_64::_mm_undefined_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_undefined_pd.html)
- [arch::x86_64::_mm_undefined_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_undefined_ps.html)
- [arch::x86_64::_mm_undefined_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_undefined_si128.html)
- [arch::x86_64::_mm_unpackhi_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_epi16.html)
- [arch::x86_64::_mm_unpackhi_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_epi32.html)
- [arch::x86_64::_mm_unpackhi_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_epi64.html)
- [arch::x86_64::_mm_unpackhi_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_epi8.html)
- [arch::x86_64::_mm_unpackhi_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_pd.html)
- [arch::x86_64::_mm_unpackhi_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_pi16.html)
- [arch::x86_64::_mm_unpackhi_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_pi32.html)
- [arch::x86_64::_mm_unpackhi_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_pi8.html)
- [arch::x86_64::_mm_unpackhi_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpackhi_ps.html)
- [arch::x86_64::_mm_unpacklo_epi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_epi16.html)
- [arch::x86_64::_mm_unpacklo_epi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_epi32.html)
- [arch::x86_64::_mm_unpacklo_epi64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_epi64.html)
- [arch::x86_64::_mm_unpacklo_epi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_epi8.html)
- [arch::x86_64::_mm_unpacklo_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_pd.html)
- [arch::x86_64::_mm_unpacklo_pi16](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_pi16.html)
- [arch::x86_64::_mm_unpacklo_pi32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_pi32.html)
- [arch::x86_64::_mm_unpacklo_pi8](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_pi8.html)
- [arch::x86_64::_mm_unpacklo_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_unpacklo_ps.html)
- [arch::x86_64::_mm_xor_pd](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_xor_pd.html)
- [arch::x86_64::_mm_xor_ps](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_xor_ps.html)
- [arch::x86_64::_mm_xor_si128](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mm_xor_si128.html)
- [arch::x86_64::_mulx_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mulx_u32.html)
- [arch::x86_64::_mulx_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._mulx_u64.html)
- [arch::x86_64::_pdep_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._pdep_u32.html)
- [arch::x86_64::_pdep_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._pdep_u64.html)
- [arch::x86_64::_pext_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._pext_u32.html)
- [arch::x86_64::_pext_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._pext_u64.html)
- [arch::x86_64::_popcnt32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._popcnt32.html)
- [arch::x86_64::_popcnt64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._popcnt64.html)
- [arch::x86_64::_rdrand16_step](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdrand16_step.html)
- [arch::x86_64::_rdrand32_step](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdrand32_step.html)
- [arch::x86_64::_rdrand64_step](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdrand64_step.html)
- [arch::x86_64::_rdseed16_step](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdseed16_step.html)
- [arch::x86_64::_rdseed32_step](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdseed32_step.html)
- [arch::x86_64::_rdseed64_step](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdseed64_step.html)
- [arch::x86_64::_rdtsc](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._rdtsc.html)
- [arch::x86_64::_t1mskc_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._t1mskc_u32.html)
- [arch::x86_64::_t1mskc_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._t1mskc_u64.html)
- [arch::x86_64::_tzcnt_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._tzcnt_u32.html)
- [arch::x86_64::_tzcnt_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._tzcnt_u64.html)
- [arch::x86_64::_tzmsk_u32](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._tzmsk_u32.html)
- [arch::x86_64::_tzmsk_u64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._tzmsk_u64.html)
- [arch::x86_64::_xgetbv](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xgetbv.html)
- [arch::x86_64::_xrstor](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xrstor.html)
- [arch::x86_64::_xrstor64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xrstor64.html)
- [arch::x86_64::_xrstors](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xrstors.html)
- [arch::x86_64::_xrstors64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xrstors64.html)
- [arch::x86_64::_xsave](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsave.html)
- [arch::x86_64::_xsave64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsave64.html)
- [arch::x86_64::_xsavec](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsavec.html)
- [arch::x86_64::_xsavec64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsavec64.html)
- [arch::x86_64::_xsaveopt](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsaveopt.html)
- [arch::x86_64::_xsaveopt64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsaveopt64.html)
- [arch::x86_64::_xsaves](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsaves.html)
- [arch::x86_64::_xsaves64](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsaves64.html)
- [arch::x86_64::_xsetbv](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn._xsetbv.html)
- [arch::x86_64::has_cpuid](https://doc.rust-lang.org/nightly/core/arch/x86_64/fn.has_cpuid.html)
- [ascii::escape_default](https://doc.rust-lang.org/nightly/core/ascii/fn.escape_default.html)
- [char::decode_utf16](https://doc.rust-lang.org/nightly/core/char/fn.decode_utf16.html)
- [char::decode_utf8](https://doc.rust-lang.org/nightly/core/char/fn.decode_utf8.html)
- [char::from_digit](https://doc.rust-lang.org/nightly/core/char/fn.from_digit.html)
- [char::from_u32](https://doc.rust-lang.org/nightly/core/char/fn.from_u32.html)
- [char::from_u32_unchecked](https://doc.rust-lang.org/nightly/core/char/fn.from_u32_unchecked.html)
- [cmp::max](https://doc.rust-lang.org/nightly/core/cmp/fn.max.html)
- [cmp::min](https://doc.rust-lang.org/nightly/core/cmp/fn.min.html)
- [fmt::write](https://doc.rust-lang.org/nightly/core/fmt/fn.write.html)
- [hint::unreachable_unchecked](https://doc.rust-lang.org/nightly/core/hint/fn.unreachable_unchecked.html)
- [intrinsics::abort](https://doc.rust-lang.org/nightly/core/intrinsics/fn.abort.html)
- [intrinsics::add_with_overflow](https://doc.rust-lang.org/nightly/core/intrinsics/fn.add_with_overflow.html)
- [intrinsics::arith_offset](https://doc.rust-lang.org/nightly/core/intrinsics/fn.arith_offset.html)
- [intrinsics::assume](https://doc.rust-lang.org/nightly/core/intrinsics/fn.assume.html)
- [intrinsics::atomic_and](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_and.html)
- [intrinsics::atomic_and_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_and_acq.html)
- [intrinsics::atomic_and_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_and_acqrel.html)
- [intrinsics::atomic_and_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_and_rel.html)
- [intrinsics::atomic_and_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_and_relaxed.html)
- [intrinsics::atomic_cxchg](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg.html)
- [intrinsics::atomic_cxchg_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_acq.html)
- [intrinsics::atomic_cxchg_acq_failrelaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_acq_failrelaxed.html)
- [intrinsics::atomic_cxchg_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_acqrel.html)
- [intrinsics::atomic_cxchg_acqrel_failrelaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_acqrel_failrelaxed.html)
- [intrinsics::atomic_cxchg_failacq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_failacq.html)
- [intrinsics::atomic_cxchg_failrelaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_failrelaxed.html)
- [intrinsics::atomic_cxchg_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_rel.html)
- [intrinsics::atomic_cxchg_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchg_relaxed.html)
- [intrinsics::atomic_cxchgweak](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak.html)
- [intrinsics::atomic_cxchgweak_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_acq.html)
- [intrinsics::atomic_cxchgweak_acq_failrelaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_acq_failrelaxed.html)
- [intrinsics::atomic_cxchgweak_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_acqrel.html)
- [intrinsics::atomic_cxchgweak_acqrel_failrelaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_acqrel_failrelaxed.html)
- [intrinsics::atomic_cxchgweak_failacq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_failacq.html)
- [intrinsics::atomic_cxchgweak_failrelaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_failrelaxed.html)
- [intrinsics::atomic_cxchgweak_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_rel.html)
- [intrinsics::atomic_cxchgweak_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_cxchgweak_relaxed.html)
- [intrinsics::atomic_fence](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_fence.html)
- [intrinsics::atomic_fence_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_fence_acq.html)
- [intrinsics::atomic_fence_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_fence_acqrel.html)
- [intrinsics::atomic_fence_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_fence_rel.html)
- [intrinsics::atomic_load](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_load.html)
- [intrinsics::atomic_load_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_load_acq.html)
- [intrinsics::atomic_load_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_load_relaxed.html)
- [intrinsics::atomic_load_unordered](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_load_unordered.html)
- [intrinsics::atomic_max](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_max.html)
- [intrinsics::atomic_max_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_max_acq.html)
- [intrinsics::atomic_max_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_max_acqrel.html)
- [intrinsics::atomic_max_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_max_rel.html)
- [intrinsics::atomic_max_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_max_relaxed.html)
- [intrinsics::atomic_min](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_min.html)
- [intrinsics::atomic_min_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_min_acq.html)
- [intrinsics::atomic_min_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_min_acqrel.html)
- [intrinsics::atomic_min_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_min_rel.html)
- [intrinsics::atomic_min_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_min_relaxed.html)
- [intrinsics::atomic_nand](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_nand.html)
- [intrinsics::atomic_nand_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_nand_acq.html)
- [intrinsics::atomic_nand_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_nand_acqrel.html)
- [intrinsics::atomic_nand_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_nand_rel.html)
- [intrinsics::atomic_nand_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_nand_relaxed.html)
- [intrinsics::atomic_or](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_or.html)
- [intrinsics::atomic_or_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_or_acq.html)
- [intrinsics::atomic_or_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_or_acqrel.html)
- [intrinsics::atomic_or_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_or_rel.html)
- [intrinsics::atomic_or_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_or_relaxed.html)
- [intrinsics::atomic_singlethreadfence](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_singlethreadfence.html)
- [intrinsics::atomic_singlethreadfence_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_singlethreadfence_acq.html)
- [intrinsics::atomic_singlethreadfence_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_singlethreadfence_acqrel.html)
- [intrinsics::atomic_singlethreadfence_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_singlethreadfence_rel.html)
- [intrinsics::atomic_store](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_store.html)
- [intrinsics::atomic_store_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_store_rel.html)
- [intrinsics::atomic_store_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_store_relaxed.html)
- [intrinsics::atomic_store_unordered](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_store_unordered.html)
- [intrinsics::atomic_umax](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umax.html)
- [intrinsics::atomic_umax_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umax_acq.html)
- [intrinsics::atomic_umax_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umax_acqrel.html)
- [intrinsics::atomic_umax_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umax_rel.html)
- [intrinsics::atomic_umax_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umax_relaxed.html)
- [intrinsics::atomic_umin](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umin.html)
- [intrinsics::atomic_umin_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umin_acq.html)
- [intrinsics::atomic_umin_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umin_acqrel.html)
- [intrinsics::atomic_umin_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umin_rel.html)
- [intrinsics::atomic_umin_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_umin_relaxed.html)
- [intrinsics::atomic_xadd](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xadd.html)
- [intrinsics::atomic_xadd_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xadd_acq.html)
- [intrinsics::atomic_xadd_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xadd_acqrel.html)
- [intrinsics::atomic_xadd_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xadd_rel.html)
- [intrinsics::atomic_xadd_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xadd_relaxed.html)
- [intrinsics::atomic_xchg](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xchg.html)
- [intrinsics::atomic_xchg_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xchg_acq.html)
- [intrinsics::atomic_xchg_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xchg_acqrel.html)
- [intrinsics::atomic_xchg_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xchg_rel.html)
- [intrinsics::atomic_xchg_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xchg_relaxed.html)
- [intrinsics::atomic_xor](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xor.html)
- [intrinsics::atomic_xor_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xor_acq.html)
- [intrinsics::atomic_xor_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xor_acqrel.html)
- [intrinsics::atomic_xor_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xor_rel.html)
- [intrinsics::atomic_xor_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xor_relaxed.html)
- [intrinsics::atomic_xsub](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xsub.html)
- [intrinsics::atomic_xsub_acq](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xsub_acq.html)
- [intrinsics::atomic_xsub_acqrel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xsub_acqrel.html)
- [intrinsics::atomic_xsub_rel](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xsub_rel.html)
- [intrinsics::atomic_xsub_relaxed](https://doc.rust-lang.org/nightly/core/intrinsics/fn.atomic_xsub_relaxed.html)
- [intrinsics::bitreverse](https://doc.rust-lang.org/nightly/core/intrinsics/fn.bitreverse.html)
- [intrinsics::breakpoint](https://doc.rust-lang.org/nightly/core/intrinsics/fn.breakpoint.html)
- [intrinsics::bswap](https://doc.rust-lang.org/nightly/core/intrinsics/fn.bswap.html)
- [intrinsics::ceilf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ceilf32.html)
- [intrinsics::ceilf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ceilf64.html)
- [intrinsics::copy](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copy.html)
- [intrinsics::copy_nonoverlapping](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copy_nonoverlapping.html)
- [intrinsics::copysignf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copysignf32.html)
- [intrinsics::copysignf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.copysignf64.html)
- [intrinsics::cosf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.cosf32.html)
- [intrinsics::cosf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.cosf64.html)
- [intrinsics::ctlz](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ctlz.html)
- [intrinsics::ctlz_nonzero](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ctlz_nonzero.html)
- [intrinsics::ctpop](https://doc.rust-lang.org/nightly/core/intrinsics/fn.ctpop.html)
- [intrinsics::cttz](https://doc.rust-lang.org/nightly/core/intrinsics/fn.cttz.html)
- [intrinsics::cttz_nonzero](https://doc.rust-lang.org/nightly/core/intrinsics/fn.cttz_nonzero.html)
- [intrinsics::discriminant_value](https://doc.rust-lang.org/nightly/core/intrinsics/fn.discriminant_value.html)
- [intrinsics::exact_div](https://doc.rust-lang.org/nightly/core/intrinsics/fn.exact_div.html)
- [intrinsics::exp2f32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.exp2f32.html)
- [intrinsics::exp2f64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.exp2f64.html)
- [intrinsics::expf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.expf32.html)
- [intrinsics::expf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.expf64.html)
- [intrinsics::fabsf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fabsf32.html)
- [intrinsics::fabsf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fabsf64.html)
- [intrinsics::fadd_fast](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fadd_fast.html)
- [intrinsics::fdiv_fast](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fdiv_fast.html)
- [intrinsics::floorf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.floorf32.html)
- [intrinsics::floorf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.floorf64.html)
- [intrinsics::fmaf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fmaf32.html)
- [intrinsics::fmaf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fmaf64.html)
- [intrinsics::fmul_fast](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fmul_fast.html)
- [intrinsics::frem_fast](https://doc.rust-lang.org/nightly/core/intrinsics/fn.frem_fast.html)
- [intrinsics::fsub_fast](https://doc.rust-lang.org/nightly/core/intrinsics/fn.fsub_fast.html)
- [intrinsics::init](https://doc.rust-lang.org/nightly/core/intrinsics/fn.init.html)
- [intrinsics::likely](https://doc.rust-lang.org/nightly/core/intrinsics/fn.likely.html)
- [intrinsics::log10f32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.log10f32.html)
- [intrinsics::log10f64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.log10f64.html)
- [intrinsics::log2f32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.log2f32.html)
- [intrinsics::log2f64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.log2f64.html)
- [intrinsics::logf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.logf32.html)
- [intrinsics::logf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.logf64.html)
- [intrinsics::min_align_of](https://doc.rust-lang.org/nightly/core/intrinsics/fn.min_align_of.html)
- [intrinsics::min_align_of_val](https://doc.rust-lang.org/nightly/core/intrinsics/fn.min_align_of_val.html)
- [intrinsics::move_val_init](https://doc.rust-lang.org/nightly/core/intrinsics/fn.move_val_init.html)
- [intrinsics::mul_with_overflow](https://doc.rust-lang.org/nightly/core/intrinsics/fn.mul_with_overflow.html)
- [intrinsics::nearbyintf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.nearbyintf32.html)
- [intrinsics::nearbyintf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.nearbyintf64.html)
- [intrinsics::needs_drop](https://doc.rust-lang.org/nightly/core/intrinsics/fn.needs_drop.html)
- [intrinsics::nontemporal_store](https://doc.rust-lang.org/nightly/core/intrinsics/fn.nontemporal_store.html)
- [intrinsics::offset](https://doc.rust-lang.org/nightly/core/intrinsics/fn.offset.html)
- [intrinsics::overflowing_add](https://doc.rust-lang.org/nightly/core/intrinsics/fn.overflowing_add.html)
- [intrinsics::overflowing_mul](https://doc.rust-lang.org/nightly/core/intrinsics/fn.overflowing_mul.html)
- [intrinsics::overflowing_sub](https://doc.rust-lang.org/nightly/core/intrinsics/fn.overflowing_sub.html)
- [intrinsics::powf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.powf32.html)
- [intrinsics::powf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.powf64.html)
- [intrinsics::powif32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.powif32.html)
- [intrinsics::powif64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.powif64.html)
- [intrinsics::pref_align_of](https://doc.rust-lang.org/nightly/core/intrinsics/fn.pref_align_of.html)
- [intrinsics::prefetch_read_data](https://doc.rust-lang.org/nightly/core/intrinsics/fn.prefetch_read_data.html)
- [intrinsics::prefetch_read_instruction](https://doc.rust-lang.org/nightly/core/intrinsics/fn.prefetch_read_instruction.html)
- [intrinsics::prefetch_write_data](https://doc.rust-lang.org/nightly/core/intrinsics/fn.prefetch_write_data.html)
- [intrinsics::prefetch_write_instruction](https://doc.rust-lang.org/nightly/core/intrinsics/fn.prefetch_write_instruction.html)
- [intrinsics::rintf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.rintf32.html)
- [intrinsics::rintf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.rintf64.html)
- [intrinsics::roundf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.roundf32.html)
- [intrinsics::roundf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.roundf64.html)
- [intrinsics::rustc_peek](https://doc.rust-lang.org/nightly/core/intrinsics/fn.rustc_peek.html)
- [intrinsics::sinf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.sinf32.html)
- [intrinsics::sinf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.sinf64.html)
- [intrinsics::size_of](https://doc.rust-lang.org/nightly/core/intrinsics/fn.size_of.html)
- [intrinsics::size_of_val](https://doc.rust-lang.org/nightly/core/intrinsics/fn.size_of_val.html)
- [intrinsics::sqrtf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.sqrtf32.html)
- [intrinsics::sqrtf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.sqrtf64.html)
- [intrinsics::sub_with_overflow](https://doc.rust-lang.org/nightly/core/intrinsics/fn.sub_with_overflow.html)
- [intrinsics::transmute](https://doc.rust-lang.org/nightly/core/intrinsics/fn.transmute.html)
- [intrinsics::truncf32](https://doc.rust-lang.org/nightly/core/intrinsics/fn.truncf32.html)
- [intrinsics::truncf64](https://doc.rust-lang.org/nightly/core/intrinsics/fn.truncf64.html)
- [intrinsics::try](https://doc.rust-lang.org/nightly/core/intrinsics/fn.try.html)
- [intrinsics::type_id](https://doc.rust-lang.org/nightly/core/intrinsics/fn.type_id.html)
- [intrinsics::type_name](https://doc.rust-lang.org/nightly/core/intrinsics/fn.type_name.html)
- [intrinsics::unchecked_div](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_div.html)
- [intrinsics::unchecked_rem](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_rem.html)
- [intrinsics::unchecked_shl](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shl.html)
- [intrinsics::unchecked_shr](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unchecked_shr.html)
- [intrinsics::uninit](https://doc.rust-lang.org/nightly/core/intrinsics/fn.uninit.html)
- [intrinsics::unlikely](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unlikely.html)
- [intrinsics::unreachable](https://doc.rust-lang.org/nightly/core/intrinsics/fn.unreachable.html)
- [intrinsics::volatile_copy_memory](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_copy_memory.html)
- [intrinsics::volatile_copy_nonoverlapping_memory](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_copy_nonoverlapping_memory.html)
- [intrinsics::volatile_load](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_load.html)
- [intrinsics::volatile_set_memory](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_set_memory.html)
- [intrinsics::volatile_store](https://doc.rust-lang.org/nightly/core/intrinsics/fn.volatile_store.html)
- [intrinsics::write_bytes](https://doc.rust-lang.org/nightly/core/intrinsics/fn.write_bytes.html)
- [iter::empty](https://doc.rust-lang.org/nightly/core/iter/fn.empty.html)
- [iter::once](https://doc.rust-lang.org/nightly/core/iter/fn.once.html)
- [iter::repeat](https://doc.rust-lang.org/nightly/core/iter/fn.repeat.html)
- [iter::repeat_with](https://doc.rust-lang.org/nightly/core/iter/fn.repeat_with.html)
- [mem::align_of](https://doc.rust-lang.org/nightly/core/mem/fn.align_of.html)
- [mem::align_of_val](https://doc.rust-lang.org/nightly/core/mem/fn.align_of_val.html)
- [mem::discriminant](https://doc.rust-lang.org/nightly/core/mem/fn.discriminant.html)
- [mem::drop](https://doc.rust-lang.org/nightly/core/mem/fn.drop.html)
- [mem::forget](https://doc.rust-lang.org/nightly/core/mem/fn.forget.html)
- [mem::min_align_of](https://doc.rust-lang.org/nightly/core/mem/fn.min_align_of.html)
- [mem::min_align_of_val](https://doc.rust-lang.org/nightly/core/mem/fn.min_align_of_val.html)
- [mem::needs_drop](https://doc.rust-lang.org/nightly/core/mem/fn.needs_drop.html)
- [mem::replace](https://doc.rust-lang.org/nightly/core/mem/fn.replace.html)
- [mem::size_of](https://doc.rust-lang.org/nightly/core/mem/fn.size_of.html)
- [mem::size_of_val](https://doc.rust-lang.org/nightly/core/mem/fn.size_of_val.html)
- [mem::swap](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html)
- [mem::transmute_copy](https://doc.rust-lang.org/nightly/core/mem/fn.transmute_copy.html)
- [mem::uninitialized](https://doc.rust-lang.org/nightly/core/mem/fn.uninitialized.html)
- [mem::zeroed](https://doc.rust-lang.org/nightly/core/mem/fn.zeroed.html)
- [panicking::panic](https://doc.rust-lang.org/nightly/core/panicking/fn.panic.html)
- [panicking::panic_fmt](https://doc.rust-lang.org/nightly/core/panicking/fn.panic_fmt.html)
- [ptr::drop_in_place](https://doc.rust-lang.org/nightly/core/ptr/fn.drop_in_place.html)
- [ptr::eq](https://doc.rust-lang.org/nightly/core/ptr/fn.eq.html)
- [ptr::null](https://doc.rust-lang.org/nightly/core/ptr/fn.null.html)
- [ptr::null_mut](https://doc.rust-lang.org/nightly/core/ptr/fn.null_mut.html)
- [ptr::read](https://doc.rust-lang.org/nightly/core/ptr/fn.read.html)
- [ptr::read_unaligned](https://doc.rust-lang.org/nightly/core/ptr/fn.read_unaligned.html)
- [ptr::read_volatile](https://doc.rust-lang.org/nightly/core/ptr/fn.read_volatile.html)
- [ptr::replace](https://doc.rust-lang.org/nightly/core/ptr/fn.replace.html)
- [ptr::swap](https://doc.rust-lang.org/nightly/core/ptr/fn.swap.html)
- [ptr::swap_nonoverlapping](https://doc.rust-lang.org/nightly/core/ptr/fn.swap_nonoverlapping.html)
- [ptr::write](https://doc.rust-lang.org/nightly/core/ptr/fn.write.html)
- [ptr::write_unaligned](https://doc.rust-lang.org/nightly/core/ptr/fn.write_unaligned.html)
- [ptr::write_volatile](https://doc.rust-lang.org/nightly/core/ptr/fn.write_volatile.html)
- [slice::from_mut](https://doc.rust-lang.org/nightly/core/slice/fn.from_mut.html)
- [slice::from_raw_parts](https://doc.rust-lang.org/nightly/core/slice/fn.from_raw_parts.html)
- [slice::from_raw_parts_mut](https://doc.rust-lang.org/nightly/core/slice/fn.from_raw_parts_mut.html)
- [slice::from_ref](https://doc.rust-lang.org/nightly/core/slice/fn.from_ref.html)
- [slice::memchr::memchr](https://doc.rust-lang.org/nightly/core/slice/memchr/fn.memchr.html)
- [slice::memchr::memrchr](https://doc.rust-lang.org/nightly/core/slice/memchr/fn.memrchr.html)
- [str::from_utf8](https://doc.rust-lang.org/nightly/core/str/fn.from_utf8.html)
- [str::from_utf8_mut](https://doc.rust-lang.org/nightly/core/str/fn.from_utf8_mut.html)
- [str::from_utf8_unchecked](https://doc.rust-lang.org/nightly/core/str/fn.from_utf8_unchecked.html)
- [str::from_utf8_unchecked_mut](https://doc.rust-lang.org/nightly/core/str/fn.from_utf8_unchecked_mut.html)
- [str::next_code_point](https://doc.rust-lang.org/nightly/core/str/fn.next_code_point.html)
- [str::utf8_char_width](https://doc.rust-lang.org/nightly/core/str/fn.utf8_char_width.html)
- [sync::atomic::compiler_fence](https://doc.rust-lang.org/nightly/core/sync/atomic/fn.compiler_fence.html)
- [sync::atomic::fence](https://doc.rust-lang.org/nightly/core/sync/atomic/fn.fence.html)
- [sync::atomic::spin_loop_hint](https://doc.rust-lang.org/nightly/core/sync/atomic/fn.spin_loop_hint.html)
- [unicode::derived_property::Case_Ignorable](https://doc.rust-lang.org/nightly/core/unicode/derived_property/fn.Case_Ignorable.html)
- [unicode::derived_property::Cased](https://doc.rust-lang.org/nightly/core/unicode/derived_property/fn.Cased.html)
- [unicode::property::Pattern_White_Space](https://doc.rust-lang.org/nightly/core/unicode/property/fn.Pattern_White_Space.html)
- [fmt::Result](https://doc.rust-lang.org/nightly/core/fmt/type.Result.html)
- [arch::x86::_CMP_EQ_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_EQ_OQ.html)
- [arch::x86::_CMP_EQ_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_EQ_OS.html)
- [arch::x86::_CMP_EQ_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_EQ_UQ.html)
- [arch::x86::_CMP_EQ_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_EQ_US.html)
- [arch::x86::_CMP_FALSE_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_FALSE_OQ.html)
- [arch::x86::_CMP_FALSE_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_FALSE_OS.html)
- [arch::x86::_CMP_GE_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_GE_OQ.html)
- [arch::x86::_CMP_GE_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_GE_OS.html)
- [arch::x86::_CMP_GT_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_GT_OQ.html)
- [arch::x86::_CMP_GT_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_GT_OS.html)
- [arch::x86::_CMP_LE_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_LE_OQ.html)
- [arch::x86::_CMP_LE_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_LE_OS.html)
- [arch::x86::_CMP_LT_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_LT_OQ.html)
- [arch::x86::_CMP_LT_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_LT_OS.html)
- [arch::x86::_CMP_NEQ_OQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NEQ_OQ.html)
- [arch::x86::_CMP_NEQ_OS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NEQ_OS.html)
- [arch::x86::_CMP_NEQ_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NEQ_UQ.html)
- [arch::x86::_CMP_NEQ_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NEQ_US.html)
- [arch::x86::_CMP_NGE_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NGE_UQ.html)
- [arch::x86::_CMP_NGE_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NGE_US.html)
- [arch::x86::_CMP_NGT_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NGT_UQ.html)
- [arch::x86::_CMP_NGT_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NGT_US.html)
- [arch::x86::_CMP_NLE_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NLE_UQ.html)
- [arch::x86::_CMP_NLE_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NLE_US.html)
- [arch::x86::_CMP_NLT_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NLT_UQ.html)
- [arch::x86::_CMP_NLT_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_NLT_US.html)
- [arch::x86::_CMP_ORD_Q](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_ORD_Q.html)
- [arch::x86::_CMP_ORD_S](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_ORD_S.html)
- [arch::x86::_CMP_TRUE_UQ](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_TRUE_UQ.html)
- [arch::x86::_CMP_TRUE_US](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_TRUE_US.html)
- [arch::x86::_CMP_UNORD_Q](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_UNORD_Q.html)
- [arch::x86::_CMP_UNORD_S](https://doc.rust-lang.org/nightly/core/arch/x86/constant._CMP_UNORD_S.html)
- [arch::x86::_MM_EXCEPT_DENORM](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_DENORM.html)
- [arch::x86::_MM_EXCEPT_DIV_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_DIV_ZERO.html)
- [arch::x86::_MM_EXCEPT_INEXACT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_INEXACT.html)
- [arch::x86::_MM_EXCEPT_INVALID](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_INVALID.html)
- [arch::x86::_MM_EXCEPT_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_MASK.html)
- [arch::x86::_MM_EXCEPT_OVERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_OVERFLOW.html)
- [arch::x86::_MM_EXCEPT_UNDERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_EXCEPT_UNDERFLOW.html)
- [arch::x86::_MM_FLUSH_ZERO_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FLUSH_ZERO_MASK.html)
- [arch::x86::_MM_FLUSH_ZERO_OFF](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FLUSH_ZERO_OFF.html)
- [arch::x86::_MM_FLUSH_ZERO_ON](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FLUSH_ZERO_ON.html)
- [arch::x86::_MM_FROUND_CEIL](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_CEIL.html)
- [arch::x86::_MM_FROUND_CUR_DIRECTION](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_CUR_DIRECTION.html)
- [arch::x86::_MM_FROUND_FLOOR](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_FLOOR.html)
- [arch::x86::_MM_FROUND_NEARBYINT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_NEARBYINT.html)
- [arch::x86::_MM_FROUND_NINT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_NINT.html)
- [arch::x86::_MM_FROUND_NO_EXC](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_NO_EXC.html)
- [arch::x86::_MM_FROUND_RAISE_EXC](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_RAISE_EXC.html)
- [arch::x86::_MM_FROUND_RINT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_RINT.html)
- [arch::x86::_MM_FROUND_TO_NEAREST_INT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_TO_NEAREST_INT.html)
- [arch::x86::_MM_FROUND_TO_NEG_INF](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_TO_NEG_INF.html)
- [arch::x86::_MM_FROUND_TO_POS_INF](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_TO_POS_INF.html)
- [arch::x86::_MM_FROUND_TO_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_TO_ZERO.html)
- [arch::x86::_MM_FROUND_TRUNC](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_FROUND_TRUNC.html)
- [arch::x86::_MM_HINT_NTA](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_HINT_NTA.html)
- [arch::x86::_MM_HINT_T0](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_HINT_T0.html)
- [arch::x86::_MM_HINT_T1](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_HINT_T1.html)
- [arch::x86::_MM_HINT_T2](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_HINT_T2.html)
- [arch::x86::_MM_MASK_DENORM](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_DENORM.html)
- [arch::x86::_MM_MASK_DIV_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_DIV_ZERO.html)
- [arch::x86::_MM_MASK_INEXACT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_INEXACT.html)
- [arch::x86::_MM_MASK_INVALID](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_INVALID.html)
- [arch::x86::_MM_MASK_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_MASK.html)
- [arch::x86::_MM_MASK_OVERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_OVERFLOW.html)
- [arch::x86::_MM_MASK_UNDERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_MASK_UNDERFLOW.html)
- [arch::x86::_MM_ROUND_DOWN](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_ROUND_DOWN.html)
- [arch::x86::_MM_ROUND_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_ROUND_MASK.html)
- [arch::x86::_MM_ROUND_NEAREST](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_ROUND_NEAREST.html)
- [arch::x86::_MM_ROUND_TOWARD_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_ROUND_TOWARD_ZERO.html)
- [arch::x86::_MM_ROUND_UP](https://doc.rust-lang.org/nightly/core/arch/x86/constant._MM_ROUND_UP.html)
- [arch::x86::_SIDD_BIT_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_BIT_MASK.html)
- [arch::x86::_SIDD_CMP_EQUAL_ANY](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_CMP_EQUAL_ANY.html)
- [arch::x86::_SIDD_CMP_EQUAL_EACH](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_CMP_EQUAL_EACH.html)
- [arch::x86::_SIDD_CMP_EQUAL_ORDERED](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_CMP_EQUAL_ORDERED.html)
- [arch::x86::_SIDD_CMP_RANGES](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_CMP_RANGES.html)
- [arch::x86::_SIDD_LEAST_SIGNIFICANT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_LEAST_SIGNIFICANT.html)
- [arch::x86::_SIDD_MASKED_NEGATIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_MASKED_NEGATIVE_POLARITY.html)
- [arch::x86::_SIDD_MASKED_POSITIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_MASKED_POSITIVE_POLARITY.html)
- [arch::x86::_SIDD_MOST_SIGNIFICANT](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_MOST_SIGNIFICANT.html)
- [arch::x86::_SIDD_NEGATIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_NEGATIVE_POLARITY.html)
- [arch::x86::_SIDD_POSITIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_POSITIVE_POLARITY.html)
- [arch::x86::_SIDD_SBYTE_OPS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_SBYTE_OPS.html)
- [arch::x86::_SIDD_SWORD_OPS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_SWORD_OPS.html)
- [arch::x86::_SIDD_UBYTE_OPS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_UBYTE_OPS.html)
- [arch::x86::_SIDD_UNIT_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_UNIT_MASK.html)
- [arch::x86::_SIDD_UWORD_OPS](https://doc.rust-lang.org/nightly/core/arch/x86/constant._SIDD_UWORD_OPS.html)
- [arch::x86::_XCR_XFEATURE_ENABLED_MASK](https://doc.rust-lang.org/nightly/core/arch/x86/constant._XCR_XFEATURE_ENABLED_MASK.html)
- [arch::x86_64::_CMP_EQ_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_EQ_OQ.html)
- [arch::x86_64::_CMP_EQ_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_EQ_OS.html)
- [arch::x86_64::_CMP_EQ_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_EQ_UQ.html)
- [arch::x86_64::_CMP_EQ_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_EQ_US.html)
- [arch::x86_64::_CMP_FALSE_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_FALSE_OQ.html)
- [arch::x86_64::_CMP_FALSE_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_FALSE_OS.html)
- [arch::x86_64::_CMP_GE_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_GE_OQ.html)
- [arch::x86_64::_CMP_GE_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_GE_OS.html)
- [arch::x86_64::_CMP_GT_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_GT_OQ.html)
- [arch::x86_64::_CMP_GT_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_GT_OS.html)
- [arch::x86_64::_CMP_LE_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_LE_OQ.html)
- [arch::x86_64::_CMP_LE_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_LE_OS.html)
- [arch::x86_64::_CMP_LT_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_LT_OQ.html)
- [arch::x86_64::_CMP_LT_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_LT_OS.html)
- [arch::x86_64::_CMP_NEQ_OQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NEQ_OQ.html)
- [arch::x86_64::_CMP_NEQ_OS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NEQ_OS.html)
- [arch::x86_64::_CMP_NEQ_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NEQ_UQ.html)
- [arch::x86_64::_CMP_NEQ_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NEQ_US.html)
- [arch::x86_64::_CMP_NGE_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NGE_UQ.html)
- [arch::x86_64::_CMP_NGE_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NGE_US.html)
- [arch::x86_64::_CMP_NGT_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NGT_UQ.html)
- [arch::x86_64::_CMP_NGT_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NGT_US.html)
- [arch::x86_64::_CMP_NLE_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NLE_UQ.html)
- [arch::x86_64::_CMP_NLE_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NLE_US.html)
- [arch::x86_64::_CMP_NLT_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NLT_UQ.html)
- [arch::x86_64::_CMP_NLT_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_NLT_US.html)
- [arch::x86_64::_CMP_ORD_Q](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_ORD_Q.html)
- [arch::x86_64::_CMP_ORD_S](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_ORD_S.html)
- [arch::x86_64::_CMP_TRUE_UQ](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_TRUE_UQ.html)
- [arch::x86_64::_CMP_TRUE_US](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_TRUE_US.html)
- [arch::x86_64::_CMP_UNORD_Q](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_UNORD_Q.html)
- [arch::x86_64::_CMP_UNORD_S](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._CMP_UNORD_S.html)
- [arch::x86_64::_MM_EXCEPT_DENORM](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_DENORM.html)
- [arch::x86_64::_MM_EXCEPT_DIV_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_DIV_ZERO.html)
- [arch::x86_64::_MM_EXCEPT_INEXACT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_INEXACT.html)
- [arch::x86_64::_MM_EXCEPT_INVALID](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_INVALID.html)
- [arch::x86_64::_MM_EXCEPT_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_MASK.html)
- [arch::x86_64::_MM_EXCEPT_OVERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_OVERFLOW.html)
- [arch::x86_64::_MM_EXCEPT_UNDERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_EXCEPT_UNDERFLOW.html)
- [arch::x86_64::_MM_FLUSH_ZERO_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FLUSH_ZERO_MASK.html)
- [arch::x86_64::_MM_FLUSH_ZERO_OFF](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FLUSH_ZERO_OFF.html)
- [arch::x86_64::_MM_FLUSH_ZERO_ON](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FLUSH_ZERO_ON.html)
- [arch::x86_64::_MM_FROUND_CEIL](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_CEIL.html)
- [arch::x86_64::_MM_FROUND_CUR_DIRECTION](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_CUR_DIRECTION.html)
- [arch::x86_64::_MM_FROUND_FLOOR](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_FLOOR.html)
- [arch::x86_64::_MM_FROUND_NEARBYINT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_NEARBYINT.html)
- [arch::x86_64::_MM_FROUND_NINT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_NINT.html)
- [arch::x86_64::_MM_FROUND_NO_EXC](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_NO_EXC.html)
- [arch::x86_64::_MM_FROUND_RAISE_EXC](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_RAISE_EXC.html)
- [arch::x86_64::_MM_FROUND_RINT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_RINT.html)
- [arch::x86_64::_MM_FROUND_TO_NEAREST_INT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_TO_NEAREST_INT.html)
- [arch::x86_64::_MM_FROUND_TO_NEG_INF](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_TO_NEG_INF.html)
- [arch::x86_64::_MM_FROUND_TO_POS_INF](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_TO_POS_INF.html)
- [arch::x86_64::_MM_FROUND_TO_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_TO_ZERO.html)
- [arch::x86_64::_MM_FROUND_TRUNC](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_FROUND_TRUNC.html)
- [arch::x86_64::_MM_HINT_NTA](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_HINT_NTA.html)
- [arch::x86_64::_MM_HINT_T0](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_HINT_T0.html)
- [arch::x86_64::_MM_HINT_T1](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_HINT_T1.html)
- [arch::x86_64::_MM_HINT_T2](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_HINT_T2.html)
- [arch::x86_64::_MM_MASK_DENORM](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_DENORM.html)
- [arch::x86_64::_MM_MASK_DIV_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_DIV_ZERO.html)
- [arch::x86_64::_MM_MASK_INEXACT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_INEXACT.html)
- [arch::x86_64::_MM_MASK_INVALID](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_INVALID.html)
- [arch::x86_64::_MM_MASK_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_MASK.html)
- [arch::x86_64::_MM_MASK_OVERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_OVERFLOW.html)
- [arch::x86_64::_MM_MASK_UNDERFLOW](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_MASK_UNDERFLOW.html)
- [arch::x86_64::_MM_ROUND_DOWN](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_ROUND_DOWN.html)
- [arch::x86_64::_MM_ROUND_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_ROUND_MASK.html)
- [arch::x86_64::_MM_ROUND_NEAREST](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_ROUND_NEAREST.html)
- [arch::x86_64::_MM_ROUND_TOWARD_ZERO](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_ROUND_TOWARD_ZERO.html)
- [arch::x86_64::_MM_ROUND_UP](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._MM_ROUND_UP.html)
- [arch::x86_64::_SIDD_BIT_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_BIT_MASK.html)
- [arch::x86_64::_SIDD_CMP_EQUAL_ANY](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_CMP_EQUAL_ANY.html)
- [arch::x86_64::_SIDD_CMP_EQUAL_EACH](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_CMP_EQUAL_EACH.html)
- [arch::x86_64::_SIDD_CMP_EQUAL_ORDERED](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_CMP_EQUAL_ORDERED.html)
- [arch::x86_64::_SIDD_CMP_RANGES](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_CMP_RANGES.html)
- [arch::x86_64::_SIDD_LEAST_SIGNIFICANT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_LEAST_SIGNIFICANT.html)
- [arch::x86_64::_SIDD_MASKED_NEGATIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_MASKED_NEGATIVE_POLARITY.html)
- [arch::x86_64::_SIDD_MASKED_POSITIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_MASKED_POSITIVE_POLARITY.html)
- [arch::x86_64::_SIDD_MOST_SIGNIFICANT](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_MOST_SIGNIFICANT.html)
- [arch::x86_64::_SIDD_NEGATIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_NEGATIVE_POLARITY.html)
- [arch::x86_64::_SIDD_POSITIVE_POLARITY](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_POSITIVE_POLARITY.html)
- [arch::x86_64::_SIDD_SBYTE_OPS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_SBYTE_OPS.html)
- [arch::x86_64::_SIDD_SWORD_OPS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_SWORD_OPS.html)
- [arch::x86_64::_SIDD_UBYTE_OPS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_UBYTE_OPS.html)
- [arch::x86_64::_SIDD_UNIT_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_UNIT_MASK.html)
- [arch::x86_64::_SIDD_UWORD_OPS](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._SIDD_UWORD_OPS.html)
- [arch::x86_64::_XCR_XFEATURE_ENABLED_MASK](https://doc.rust-lang.org/nightly/core/arch/x86_64/constant._XCR_XFEATURE_ENABLED_MASK.html)
- [char::MAX](https://doc.rust-lang.org/nightly/core/char/constant.MAX.html)
- [char::REPLACEMENT_CHARACTER](https://doc.rust-lang.org/nightly/core/char/constant.REPLACEMENT_CHARACTER.html)
- [char::UNICODE_VERSION](https://doc.rust-lang.org/nightly/core/char/constant.UNICODE_VERSION.html)
- [f32::DIGITS](https://doc.rust-lang.org/nightly/core/f32/constant.DIGITS.html)
- [f32::EPSILON](https://doc.rust-lang.org/nightly/core/f32/constant.EPSILON.html)
- [f32::INFINITY](https://doc.rust-lang.org/nightly/core/f32/constant.INFINITY.html)
- [f32::MANTISSA_DIGITS](https://doc.rust-lang.org/nightly/core/f32/constant.MANTISSA_DIGITS.html)
- [f32::MAX](https://doc.rust-lang.org/nightly/core/f32/constant.MAX.html)
- [f32::MAX_10_EXP](https://doc.rust-lang.org/nightly/core/f32/constant.MAX_10_EXP.html)
- [f32::MAX_EXP](https://doc.rust-lang.org/nightly/core/f32/constant.MAX_EXP.html)
- [f32::MIN](https://doc.rust-lang.org/nightly/core/f32/constant.MIN.html)
- [f32::MIN_10_EXP](https://doc.rust-lang.org/nightly/core/f32/constant.MIN_10_EXP.html)
- [f32::MIN_EXP](https://doc.rust-lang.org/nightly/core/f32/constant.MIN_EXP.html)
- [f32::MIN_POSITIVE](https://doc.rust-lang.org/nightly/core/f32/constant.MIN_POSITIVE.html)
- [f32::NAN](https://doc.rust-lang.org/nightly/core/f32/constant.NAN.html)
- [f32::NEG_INFINITY](https://doc.rust-lang.org/nightly/core/f32/constant.NEG_INFINITY.html)
- [f32::RADIX](https://doc.rust-lang.org/nightly/core/f32/constant.RADIX.html)
- [f32::consts::E](https://doc.rust-lang.org/nightly/core/f32/consts/constant.E.html)
- [f32::consts::FRAC_1_PI](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_1_PI.html)
- [f32::consts::FRAC_1_SQRT_2](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_1_SQRT_2.html)
- [f32::consts::FRAC_2_PI](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_2_PI.html)
- [f32::consts::FRAC_2_SQRT_PI](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_2_SQRT_PI.html)
- [f32::consts::FRAC_PI_2](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_PI_2.html)
- [f32::consts::FRAC_PI_3](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_PI_3.html)
- [f32::consts::FRAC_PI_4](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_PI_4.html)
- [f32::consts::FRAC_PI_6](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_PI_6.html)
- [f32::consts::FRAC_PI_8](https://doc.rust-lang.org/nightly/core/f32/consts/constant.FRAC_PI_8.html)
- [f32::consts::LN_10](https://doc.rust-lang.org/nightly/core/f32/consts/constant.LN_10.html)
- [f32::consts::LN_2](https://doc.rust-lang.org/nightly/core/f32/consts/constant.LN_2.html)
- [f32::consts::LOG10_2](https://doc.rust-lang.org/nightly/core/f32/consts/constant.LOG10_2.html)
- [f32::consts::LOG10_E](https://doc.rust-lang.org/nightly/core/f32/consts/constant.LOG10_E.html)
- [f32::consts::LOG2_10](https://doc.rust-lang.org/nightly/core/f32/consts/constant.LOG2_10.html)
- [f32::consts::LOG2_E](https://doc.rust-lang.org/nightly/core/f32/consts/constant.LOG2_E.html)
- [f32::consts::PI](https://doc.rust-lang.org/nightly/core/f32/consts/constant.PI.html)
- [f32::consts::SQRT_2](https://doc.rust-lang.org/nightly/core/f32/consts/constant.SQRT_2.html)
- [f64::DIGITS](https://doc.rust-lang.org/nightly/core/f64/constant.DIGITS.html)
- [f64::EPSILON](https://doc.rust-lang.org/nightly/core/f64/constant.EPSILON.html)
- [f64::INFINITY](https://doc.rust-lang.org/nightly/core/f64/constant.INFINITY.html)
- [f64::MANTISSA_DIGITS](https://doc.rust-lang.org/nightly/core/f64/constant.MANTISSA_DIGITS.html)
- [f64::MAX](https://doc.rust-lang.org/nightly/core/f64/constant.MAX.html)
- [f64::MAX_10_EXP](https://doc.rust-lang.org/nightly/core/f64/constant.MAX_10_EXP.html)
- [f64::MAX_EXP](https://doc.rust-lang.org/nightly/core/f64/constant.MAX_EXP.html)
- [f64::MIN](https://doc.rust-lang.org/nightly/core/f64/constant.MIN.html)
- [f64::MIN_10_EXP](https://doc.rust-lang.org/nightly/core/f64/constant.MIN_10_EXP.html)
- [f64::MIN_EXP](https://doc.rust-lang.org/nightly/core/f64/constant.MIN_EXP.html)
- [f64::MIN_POSITIVE](https://doc.rust-lang.org/nightly/core/f64/constant.MIN_POSITIVE.html)
- [f64::NAN](https://doc.rust-lang.org/nightly/core/f64/constant.NAN.html)
- [f64::NEG_INFINITY](https://doc.rust-lang.org/nightly/core/f64/constant.NEG_INFINITY.html)
- [f64::RADIX](https://doc.rust-lang.org/nightly/core/f64/constant.RADIX.html)
- [f64::consts::E](https://doc.rust-lang.org/nightly/core/f64/consts/constant.E.html)
- [f64::consts::FRAC_1_PI](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_1_PI.html)
- [f64::consts::FRAC_1_SQRT_2](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_1_SQRT_2.html)
- [f64::consts::FRAC_2_PI](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_2_PI.html)
- [f64::consts::FRAC_2_SQRT_PI](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_2_SQRT_PI.html)
- [f64::consts::FRAC_PI_2](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_PI_2.html)
- [f64::consts::FRAC_PI_3](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_PI_3.html)
- [f64::consts::FRAC_PI_4](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_PI_4.html)
- [f64::consts::FRAC_PI_6](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_PI_6.html)
- [f64::consts::FRAC_PI_8](https://doc.rust-lang.org/nightly/core/f64/consts/constant.FRAC_PI_8.html)
- [f64::consts::LN_10](https://doc.rust-lang.org/nightly/core/f64/consts/constant.LN_10.html)
- [f64::consts::LN_2](https://doc.rust-lang.org/nightly/core/f64/consts/constant.LN_2.html)
- [f64::consts::LOG10_2](https://doc.rust-lang.org/nightly/core/f64/consts/constant.LOG10_2.html)
- [f64::consts::LOG10_E](https://doc.rust-lang.org/nightly/core/f64/consts/constant.LOG10_E.html)
- [f64::consts::LOG2_10](https://doc.rust-lang.org/nightly/core/f64/consts/constant.LOG2_10.html)
- [f64::consts::LOG2_E](https://doc.rust-lang.org/nightly/core/f64/consts/constant.LOG2_E.html)
- [f64::consts::PI](https://doc.rust-lang.org/nightly/core/f64/consts/constant.PI.html)
- [f64::consts::SQRT_2](https://doc.rust-lang.org/nightly/core/f64/consts/constant.SQRT_2.html)
- [i128::MAX](https://doc.rust-lang.org/nightly/core/i128/constant.MAX.html)
- [i128::MIN](https://doc.rust-lang.org/nightly/core/i128/constant.MIN.html)
- [i16::MAX](https://doc.rust-lang.org/nightly/core/i16/constant.MAX.html)
- [i16::MIN](https://doc.rust-lang.org/nightly/core/i16/constant.MIN.html)
- [i32::MAX](https://doc.rust-lang.org/nightly/core/i32/constant.MAX.html)
- [i32::MIN](https://doc.rust-lang.org/nightly/core/i32/constant.MIN.html)
- [i64::MAX](https://doc.rust-lang.org/nightly/core/i64/constant.MAX.html)
- [i64::MIN](https://doc.rust-lang.org/nightly/core/i64/constant.MIN.html)
- [i8::MAX](https://doc.rust-lang.org/nightly/core/i8/constant.MAX.html)
- [i8::MIN](https://doc.rust-lang.org/nightly/core/i8/constant.MIN.html)
- [isize::MAX](https://doc.rust-lang.org/nightly/core/isize/constant.MAX.html)
- [isize::MIN](https://doc.rust-lang.org/nightly/core/isize/constant.MIN.html)
- [sync::atomic::ATOMIC_BOOL_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_BOOL_INIT.html)
- [sync::atomic::ATOMIC_I16_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_I16_INIT.html)
- [sync::atomic::ATOMIC_I32_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_I32_INIT.html)
- [sync::atomic::ATOMIC_I64_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_I64_INIT.html)
- [sync::atomic::ATOMIC_I8_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_I8_INIT.html)
- [sync::atomic::ATOMIC_ISIZE_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_ISIZE_INIT.html)
- [sync::atomic::ATOMIC_U16_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_U16_INIT.html)
- [sync::atomic::ATOMIC_U32_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_U32_INIT.html)
- [sync::atomic::ATOMIC_U64_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_U64_INIT.html)
- [sync::atomic::ATOMIC_U8_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_U8_INIT.html)
- [sync::atomic::ATOMIC_USIZE_INIT](https://doc.rust-lang.org/nightly/core/sync/atomic/constant.ATOMIC_USIZE_INIT.html)
- [u128::MAX](https://doc.rust-lang.org/nightly/core/u128/constant.MAX.html)
- [u128::MIN](https://doc.rust-lang.org/nightly/core/u128/constant.MIN.html)
- [u16::MAX](https://doc.rust-lang.org/nightly/core/u16/constant.MAX.html)
- [u16::MIN](https://doc.rust-lang.org/nightly/core/u16/constant.MIN.html)
- [u32::MAX](https://doc.rust-lang.org/nightly/core/u32/constant.MAX.html)
- [u32::MIN](https://doc.rust-lang.org/nightly/core/u32/constant.MIN.html)
- [u64::MAX](https://doc.rust-lang.org/nightly/core/u64/constant.MAX.html)
- [u64::MIN](https://doc.rust-lang.org/nightly/core/u64/constant.MIN.html)
- [u8::MAX](https://doc.rust-lang.org/nightly/core/u8/constant.MAX.html)
- [u8::MIN](https://doc.rust-lang.org/nightly/core/u8/constant.MIN.html)
- [usize::MAX](https://doc.rust-lang.org/nightly/core/usize/constant.MAX.html)
- [usize::MIN](https://doc.rust-lang.org/nightly/core/usize/constant.MIN.html)

