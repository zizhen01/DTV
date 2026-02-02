use std::fmt;

#[derive(Debug, Clone)]
pub struct DecodeErr(pub String);

impl fmt::Display for DecodeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DecodeErr {}

#[derive(Debug, Clone)]
pub struct EncodeErr(pub String);

impl fmt::Display for EncodeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for EncodeErr {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FieldType {
    Byte = 0,
    Short = 1,
    Int = 2,
    Long = 3,
    Float = 4,
    Double = 5,
    String1 = 6,
    String4 = 7,
    Map = 8,
    List = 9,
    StructBegin = 10,
    StructEnd = 11,
    ZeroTag = 12,
    SimpleList = 13,
}

fn parse_head(data: &[u8], pos: &mut usize) -> Result<(u8, FieldType), DecodeErr> {
    if *pos >= data.len() {
        return Err(DecodeErr("unexpected EOF while reading head".to_string()));
    }
    let b = data[*pos];
    *pos += 1;
    let ty = (b & 0x0F) as u8;
    let mut tag = (b >> 4) as u8;
    if tag == 15 {
        if *pos >= data.len() {
            return Err(DecodeErr("unexpected EOF while reading extended tag".to_string()));
        }
        tag = data[*pos];
        *pos += 1;
    }
    let field_type = match ty {
        0 => FieldType::Byte,
        1 => FieldType::Short,
        2 => FieldType::Int,
        3 => FieldType::Long,
        4 => FieldType::Float,
        5 => FieldType::Double,
        6 => FieldType::String1,
        7 => FieldType::String4,
        8 => FieldType::Map,
        9 => FieldType::List,
        10 => FieldType::StructBegin,
        11 => FieldType::StructEnd,
        12 => FieldType::ZeroTag,
        13 => FieldType::SimpleList,
        _ => {
            return Err(DecodeErr(format!(
                "unsupported field type {}",
                ty
            )))
        }
    };
    Ok((tag, field_type))
}

fn skip_value(data: &[u8], pos: &mut usize, ty: FieldType) -> Result<(), DecodeErr> {
    match ty {
        FieldType::Byte => *pos += 1,
        FieldType::Short => *pos += 2,
        FieldType::Int => *pos += 4,
        FieldType::Long => *pos += 8,
        FieldType::Float => *pos += 4,
        FieldType::Double => *pos += 8,
        FieldType::ZeroTag => {}
        FieldType::String1 => {
            if *pos >= data.len() {
                return Err(DecodeErr("unexpected EOF while reading string1 len".to_string()));
            }
            let len = data[*pos] as usize;
            *pos += 1 + len;
        }
        FieldType::String4 => {
            if *pos + 4 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading string4 len".to_string()));
            }
            let len = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]])
                as usize;
            *pos += 4 + len;
        }
        FieldType::StructBegin => {
            loop {
                let (_t, inner_ty) = parse_head(data, pos)?;
                if inner_ty == FieldType::StructEnd {
                    break;
                }
                skip_value(data, pos, inner_ty)?;
            }
        }
        FieldType::StructEnd => {}
        FieldType::List => {
            // list: count is encoded as an int with tag=0
            let (_t, count_ty) = parse_head(data, pos)?;
            let count = read_i32_raw(data, pos, count_ty)? as usize;
            for _ in 0..count {
                let (_et, elem_ty) = parse_head(data, pos)?;
                skip_value(data, pos, elem_ty)?;
            }
        }
        FieldType::SimpleList => {
            // simple list (bytes): element head then length(int)
            let (_et, elem_ty) = parse_head(data, pos)?;
            if elem_ty != FieldType::Byte {
                return Err(DecodeErr("simple_list only supports byte".to_string()));
            }
            let (_t, len_ty) = parse_head(data, pos)?;
            let len = read_i32_raw(data, pos, len_ty)? as usize;
            *pos += len;
        }
        FieldType::Map => {
            return Err(DecodeErr("map not supported".to_string()));
        }
    }
    if *pos > data.len() {
        return Err(DecodeErr("unexpected EOF while skipping value".to_string()));
    }
    Ok(())
}

fn read_i32_raw(data: &[u8], pos: &mut usize, ty: FieldType) -> Result<i32, DecodeErr> {
    match ty {
        FieldType::ZeroTag => Ok(0),
        FieldType::Byte => {
            if *pos >= data.len() {
                return Err(DecodeErr("unexpected EOF while reading byte".to_string()));
            }
            let v = data[*pos] as i8 as i32;
            *pos += 1;
            Ok(v)
        }
        FieldType::Short => {
            if *pos + 2 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading short".to_string()));
            }
            let v = i16::from_be_bytes([data[*pos], data[*pos + 1]]) as i32;
            *pos += 2;
            Ok(v)
        }
        FieldType::Int => {
            if *pos + 4 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading int".to_string()));
            }
            let v = i32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]);
            *pos += 4;
            Ok(v)
        }
        FieldType::Long => {
            if *pos + 8 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading long".to_string()));
            }
            let v = i64::from_be_bytes([
                data[*pos],
                data[*pos + 1],
                data[*pos + 2],
                data[*pos + 3],
                data[*pos + 4],
                data[*pos + 5],
                data[*pos + 6],
                data[*pos + 7],
            ]);
            *pos += 8;
            Ok(v as i32)
        }
        _ => Err(DecodeErr(format!("type {:?} cannot be read as i32", ty))),
    }
}

fn read_i64_raw(data: &[u8], pos: &mut usize, ty: FieldType) -> Result<i64, DecodeErr> {
    match ty {
        FieldType::ZeroTag => Ok(0),
        FieldType::Byte => {
            if *pos >= data.len() {
                return Err(DecodeErr("unexpected EOF while reading byte".to_string()));
            }
            let v = data[*pos] as i8 as i64;
            *pos += 1;
            Ok(v)
        }
        FieldType::Short => {
            if *pos + 2 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading short".to_string()));
            }
            let v = i16::from_be_bytes([data[*pos], data[*pos + 1]]) as i64;
            *pos += 2;
            Ok(v)
        }
        FieldType::Int => {
            if *pos + 4 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading int".to_string()));
            }
            let v = i32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]]) as i64;
            *pos += 4;
            Ok(v)
        }
        FieldType::Long => {
            if *pos + 8 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading long".to_string()));
            }
            let v = i64::from_be_bytes([
                data[*pos],
                data[*pos + 1],
                data[*pos + 2],
                data[*pos + 3],
                data[*pos + 4],
                data[*pos + 5],
                data[*pos + 6],
                data[*pos + 7],
            ]);
            *pos += 8;
            Ok(v)
        }
        _ => Err(DecodeErr(format!("type {:?} cannot be read as i64", ty))),
    }
}

fn read_string_raw(data: &[u8], pos: &mut usize, ty: FieldType) -> Result<String, DecodeErr> {
    match ty {
        FieldType::String1 => {
            if *pos >= data.len() {
                return Err(DecodeErr("unexpected EOF while reading string1 len".to_string()));
            }
            let len = data[*pos] as usize;
            *pos += 1;
            if *pos + len > data.len() {
                return Err(DecodeErr("unexpected EOF while reading string1".to_string()));
            }
            let s = std::str::from_utf8(&data[*pos..*pos + len])
                .map_err(|e| DecodeErr(format!("invalid utf8: {}", e)))?
                .to_string();
            *pos += len;
            Ok(s)
        }
        FieldType::String4 => {
            if *pos + 4 > data.len() {
                return Err(DecodeErr("unexpected EOF while reading string4 len".to_string()));
            }
            let len = u32::from_be_bytes([data[*pos], data[*pos + 1], data[*pos + 2], data[*pos + 3]])
                as usize;
            *pos += 4;
            if *pos + len > data.len() {
                return Err(DecodeErr("unexpected EOF while reading string4".to_string()));
            }
            let s = std::str::from_utf8(&data[*pos..*pos + len])
                .map_err(|e| DecodeErr(format!("invalid utf8: {}", e)))?
                .to_string();
            *pos += len;
            Ok(s)
        }
        _ => Err(DecodeErr(format!("type {:?} cannot be read as string", ty))),
    }
}

pub trait StructFromTars: Sized {
    fn _decode_from(decoder: &mut TarsDecoder) -> Result<Self, DecodeErr>;
}

pub struct TarsDecoder<'a> {
    data: &'a [u8],
}

impl<'a> From<&'a [u8]> for TarsDecoder<'a> {
    fn from(value: &'a [u8]) -> Self {
        Self { data: value }
    }
}

impl<'a> TarsDecoder<'a> {
    fn find_field(&self, tag: u8) -> Result<Option<(FieldType, usize)>, DecodeErr> {
        let mut pos = 0usize;
        while pos < self.data.len() {
            let (t, ty) = parse_head(self.data, &mut pos)?;
            if ty == FieldType::StructEnd {
                break;
            }
            if t == tag {
                return Ok(Some((ty, pos)));
            }
            skip_value(self.data, &mut pos, ty)?;
        }
        Ok(None)
    }

    pub fn read_int32(&mut self, tag: u8, required: bool, default: i32) -> Result<i32, DecodeErr> {
        let Some((ty, mut pos)) = self.find_field(tag)? else {
            if required {
                return Err(DecodeErr(format!("missing required int32 field tag={}", tag)));
            }
            return Ok(default);
        };
        read_i32_raw(self.data, &mut pos, ty)
    }

    pub fn read_int64(&mut self, tag: u8, required: bool, default: i64) -> Result<i64, DecodeErr> {
        let Some((ty, mut pos)) = self.find_field(tag)? else {
            if required {
                return Err(DecodeErr(format!("missing required int64 field tag={}", tag)));
            }
            return Ok(default);
        };
        read_i64_raw(self.data, &mut pos, ty)
    }

    pub fn read_string(
        &mut self,
        tag: u8,
        required: bool,
        default: String,
    ) -> Result<String, DecodeErr> {
        let Some((ty, mut pos)) = self.find_field(tag)? else {
            if required {
                return Err(DecodeErr(format!("missing required string field tag={}", tag)));
            }
            return Ok(default);
        };
        read_string_raw(self.data, &mut pos, ty)
    }

    pub fn read_bytes(
        &mut self,
        tag: u8,
        required: bool,
        default: Vec<u8>,
    ) -> Result<Vec<u8>, DecodeErr> {
        let Some((ty, mut pos)) = self.find_field(tag)? else {
            if required {
                return Err(DecodeErr(format!("missing required bytes field tag={}", tag)));
            }
            return Ok(default);
        };

        match ty {
            FieldType::SimpleList => {
                let (_et, elem_ty) = parse_head(self.data, &mut pos)?;
                if elem_ty != FieldType::Byte {
                    return Err(DecodeErr("simple_list only supports byte".to_string()));
                }
                let (_t, len_ty) = parse_head(self.data, &mut pos)?;
                let len = read_i32_raw(self.data, &mut pos, len_ty)? as usize;
                if pos + len > self.data.len() {
                    return Err(DecodeErr("unexpected EOF while reading bytes".to_string()));
                }
                Ok(self.data[pos..pos + len].to_vec())
            }
            _ => Err(DecodeErr(format!("type {:?} cannot be read as bytes", ty))),
        }
    }

    pub fn read_struct<T: StructFromTars>(
        &mut self,
        tag: u8,
        required: bool,
        default: T,
    ) -> Result<T, DecodeErr> {
        let Some((ty, pos)) = self.find_field(tag)? else {
            if required {
                return Err(DecodeErr(format!("missing required struct field tag={}", tag)));
            }
            return Ok(default);
        };
        if ty != FieldType::StructBegin {
            return Err(DecodeErr(format!("field tag={} is not struct_begin", tag)));
        }

        let start = pos;
        let end = {
            let mut scan = start;
            loop {
                let (_t, inner_ty) = parse_head(self.data, &mut scan)?;
                if inner_ty == FieldType::StructEnd {
                    break scan;
                }
                skip_value(self.data, &mut scan, inner_ty)?;
            }
        };

        let struct_bytes = &self.data[start..end];
        let mut inner = TarsDecoder::from(struct_bytes);
        T::_decode_from(&mut inner)
    }
}

pub struct TarsEncoder {
    buf: Vec<u8>,
}

impl TarsEncoder {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.buf.clone()
    }

    fn write_head(&mut self, tag: u8, ty: FieldType) {
        if tag < 15 {
            self.buf.push((tag << 4) | (ty as u8));
        } else {
            self.buf.push(0xF0 | (ty as u8));
            self.buf.push(tag);
        }
    }

    pub fn write_int32(&mut self, tag: u8, value: i32) -> Result<(), EncodeErr> {
        if value == 0 {
            self.write_head(tag, FieldType::ZeroTag);
            return Ok(());
        }
        self.write_head(tag, FieldType::Int);
        self.buf.extend_from_slice(&value.to_be_bytes());
        Ok(())
    }

    pub fn write_string(&mut self, tag: u8, value: &String) -> Result<(), EncodeErr> {
        let bytes = value.as_bytes();
        if bytes.len() < 255 {
            self.write_head(tag, FieldType::String1);
            self.buf.push(bytes.len() as u8);
        } else {
            self.write_head(tag, FieldType::String4);
            let len = bytes.len() as u32;
            self.buf.extend_from_slice(&len.to_be_bytes());
        }
        self.buf.extend_from_slice(bytes);
        Ok(())
    }

    pub fn write_bytes(&mut self, tag: u8, bytes: &[u8]) -> Result<(), EncodeErr> {
        self.write_head(tag, FieldType::SimpleList);
        // element head: tag=0, type=byte
        self.write_head(0, FieldType::Byte);
        self.write_int32(0, bytes.len() as i32)?;
        self.buf.extend_from_slice(bytes);
        Ok(())
    }

    pub fn write_list(&mut self, tag: u8, items: &[String]) -> Result<(), EncodeErr> {
        self.write_head(tag, FieldType::List);
        self.write_int32(0, items.len() as i32)?;
        for item in items {
            self.write_string(0, item)?;
        }
        Ok(())
    }
}
