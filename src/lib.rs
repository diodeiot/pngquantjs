use neon::{prelude::*, types::buffer::TypedArray};

fn quantize(mut cx: FunctionContext) -> JsResult<JsObject> {
    let buffer = cx.argument::<JsBuffer>(0)?;
    let width = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let height = cx.argument::<JsNumber>(2)?.value(&mut cx) as usize;
    let speed = cx.argument::<JsNumber>(3)?.value(&mut cx) as i32;
    let q_min = cx.argument::<JsNumber>(4)?.value(&mut cx) as u8;
    let q_max = cx.argument::<JsNumber>(5)?.value(&mut cx) as u8;
    let dithering_level = cx.argument::<JsNumber>(6)?.value(&mut cx) as f32;

    let mut liq = imagequant::new();
    liq.set_speed(speed).unwrap();
    liq.set_quality(q_min, q_max).unwrap();

    //describe the bitmap
    let data = buffer.as_slice(&cx);
    let mut bitmap: Vec<imagequant::RGBA> = Vec::with_capacity(width * height);
    for i in 0..width * height {
        let index = i * 4;
        let item = imagequant::RGBA {
            r: data[index],
            g: data[index + 1],
            b: data[index + 2],
            a: data[index + 3],
        };
        bitmap.push(item);
    }
    let mut img = liq.new_image(&bitmap[..], width, height, 0.0).unwrap();

    //quantize
    let mut res = match liq.quantize(&mut img) {
        Ok(res) => res,
        Err(err) => panic!("Quantization failed, because: {err:?}"),
    };

    //enable dithering for subsequent remappings
    res.set_dithering_level(dithering_level).unwrap();

    let (palette, pixels) = res.remapped(&mut img).unwrap();

    let mut palette_data = vec![0u8; palette.len() * 4];
    for i in 0..palette.len() {
        palette_data[i * 4] = palette[i].r;
        palette_data[i * 4 + 1] = palette[i].g;
        palette_data[i * 4 + 2] = palette[i].b;
        palette_data[i * 4 + 3] = palette[i].a;
    }

    let mut palette_buffer = JsBuffer::new(&mut cx, palette.len() * 4)?;
    {
        let js_buffer = palette_buffer.as_mut_slice(&mut cx);
        js_buffer.copy_from_slice(&palette_data);
    }

    let mut pixels_buffer = JsBuffer::new(&mut cx, pixels.len())?;
    {
        let js_buffer = pixels_buffer.as_mut_slice(&mut cx);
        js_buffer.copy_from_slice(&pixels);
    }

    let quality = cx.number(res.quantization_quality().unwrap());

    let result = JsObject::new(&mut cx);
    result.set(&mut cx, "palette", palette_buffer)?;
    result.set(&mut cx, "pixels", pixels_buffer)?;
    result.set(&mut cx, "quality", quality)?;

    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("quantize", quantize)?;
    Ok(())
}
