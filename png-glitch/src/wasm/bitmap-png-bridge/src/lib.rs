#[allow(warnings)]
mod bindings;

use std::cell::RefCell;
use std::io::{Read, Write};
use png_glitch::{FilterType, PngGlitch, ScanLine};
use bindings::exports::chikoski::glitch_art::png_glitchable::FilterType as ExportedFilterType;
use bindings::exports::chikoski::glitch_art::png_glitchable::ScanLine as ExportedScanLine;
use bindings::exports::chikoski::glitch_art::png_glitchable::{GuestPng, Png, Guest, GuestScanLine};
use crate::bindings::export;

impl From<FilterType> for ExportedFilterType {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::None => ExportedFilterType::None,
            FilterType::Sub => ExportedFilterType::Sub,
            FilterType::Up => ExportedFilterType::Up,
            FilterType::Average => ExportedFilterType::Average,
            FilterType::Paeth => ExportedFilterType::Paeth,
        }
    }
}

impl From<ExportedFilterType> for FilterType {
    fn from(value: ExportedFilterType) -> Self {
        match value {
            ExportedFilterType::None => FilterType::None,
            ExportedFilterType::Sub => FilterType::Sub,
            ExportedFilterType::Up => FilterType::Up,
            ExportedFilterType::Average => FilterType::Average,
            ExportedFilterType::Paeth => FilterType::Paeth,
        }
    }
}

struct BitmapImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl BitmapImage {
    fn new(data: Vec<u8>, width: u32, height: u32) -> BitmapImage {
        BitmapImage { data, width, height }
    }
}

struct PngImage {
    bytes: Vec<u8>
}

impl TryFrom<BitmapImage> for PngImage {
    type Error = ();

    fn try_from(value: BitmapImage) -> Result<Self, Self::Error> {
        let mut bytes = vec![];
        let mut encoder = png::Encoder::new(&mut bytes, value.width, value.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|_| ())?;
        writer.write_image_data(&value.data).map_err(|_| ())?;
        let _ = writer.finish();
        
        let png_image_data = PngImage { bytes };
        Ok(png_image_data)
    }
}

struct GuestScanLineImpl {
    inner: RefCell<ScanLine>
}

impl GuestScanLineImpl {
    fn new(scan_line: ScanLine) -> GuestScanLineImpl {
        let inner = RefCell::new(scan_line);
        GuestScanLineImpl { inner }
    }
}

impl GuestScanLine for GuestScanLineImpl {
    fn get_filter_type(&self) -> ExportedFilterType {
        self.inner.borrow().filter_type().into()
    }

    fn set_filter_type(&self, t: ExportedFilterType) {
        self.inner.borrow_mut().set_filter_type(t.into());
    }

    fn size(&self) -> u32 {
        self.inner.borrow().size()as u32
    }

    fn get_pixel_at(&self, index: u32) -> u8 {
        self.inner.borrow()[index as usize]
    }

    fn set_pixel_at(&self, index: u32, value: u8) {
        self.inner.borrow_mut()[index as usize] = value;
    }

    fn read(&self) -> Result<Vec<u8>, ()> {
        let mut buffer = vec![];
        self.inner.borrow_mut().read(&mut buffer).map_err(|_| ())?;
        Ok(buffer)
    }

    fn write(&self, pixels: Vec<u8>) {
        let _ = self.inner.borrow_mut().write(&pixels);
    }
}

struct GuestPngImpl {
    inner: RefCell<PngGlitch>
}

impl GuestPngImpl {
    fn new(png_image: PngGlitch) -> GuestPngImpl {
        let inner = RefCell::new(png_image);
        GuestPngImpl { inner }
    }
}

impl GuestPng for GuestPngImpl {
    fn get_scan_lines(&self) -> Vec<ExportedScanLine> {
        self.inner.borrow_mut().scan_lines().into_iter().map(|actual| {
            let scanline= GuestScanLineImpl::new(actual);
            ExportedScanLine::new(scanline)
        }).collect()
    }

    fn read(&self) -> Result<Vec<u8>, ()> {
        let mut buffer = vec![];
        self.inner.borrow_mut().encode(&mut buffer).map_err(|_| ())?;
        Ok(buffer)
    }

    fn create(data: Vec<u8>, width: u32, height: u32) -> Result<Png, ()> {
        let bitmap = BitmapImage::new(data, width, height);
        let png_image = PngImage::try_from(bitmap).map_err(|_| ())?;
        let guest_png = GuestPngImpl::try_from(png_image)?;
        Ok(Png::new(guest_png))
    }
}

impl TryFrom<PngImage> for GuestPngImpl {
    type Error = ();

    fn try_from(value: PngImage) -> Result<Self, Self::Error> {
        let png_glitch = PngGlitch::new(value.bytes).map_err(|_| ())?;
        let guest_png = GuestPngImpl::new(png_glitch);
        Ok(guest_png)
    }
}

struct Component;

impl Guest for Component {
    type ScanLine = GuestScanLineImpl;
    type Png = GuestPngImpl;
}

export!(Component with_types_in bindings);
