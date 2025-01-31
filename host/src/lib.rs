#[cfg(test)]
mod wasm_wasi_test {
    use wasmtime::component::Component;
    use wasmtime::component::Linker;
    use wasmtime::component::ResourceAny;
    use wasmtime::component::ResourceTable;
    use wasmtime::component::Val;
    use wasmtime::Config;
    use wasmtime::Engine;
    use wasmtime::Store;

    const IMAGE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/cat.png"));

    const GUEST_RS_WASI_MODULE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../guest-rs/target/wasm32-wasip2/release/component.wasm"
    ));

    const GUEST_PY_WASI_COMPONENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../guest-py/component.wasm"
    ));

    const GUEST_JS_WASI_COMPONENT: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../guest-js/component.wasm"
    ));

    struct Host {
        ctx: wasmtime_wasi::WasiCtx,
        table: ResourceTable,
    }

    impl wasmtime_wasi::WasiView for Host {
        fn table(&mut self) -> &mut ResourceTable {
            &mut self.table
        }

        fn ctx(&mut self) -> &mut wasmtime_wasi::WasiCtx {
            &mut self.ctx
        }
    }

    impl Host {
        fn new() -> Self {
            let ctx = wasmtime_wasi::WasiCtxBuilder::new().inherit_stdio().build();
            let table = ResourceTable::new();
            Self { ctx, table }
        }
    }

    #[test]
    fn test_rs_guest() -> anyhow::Result<()> {
        let config = Config::new();
        let engine = Engine::new(&config).unwrap();
        let host = Host::new();
        let mut store = Store::new(&engine, host);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync::<Host>(&mut linker).unwrap();

        let component = Component::from_binary(&engine, &GUEST_RS_WASI_MODULE).unwrap();
        let instance = linker.instantiate(&mut store, &component).unwrap();
        let intf_export = instance.get_export(&mut store, None, "intf").unwrap();

        // Test Print
        let func_print_export = instance
            .get_export(&mut store, Some(&intf_export), "print")
            .unwrap();
        let func_print = instance
            .get_typed_func(&mut store, func_print_export)
            .unwrap();
        let () = func_print
            .call(&mut store, ("Hello world".to_string(),))
            .unwrap();
        func_print.post_return(&mut store).unwrap();

        // Test Extract Emails

        let func_extract_emails_export = instance
            .get_export(&mut store, Some(&intf_export), "extract-emails")
            .unwrap();
        let func_extract_emails = instance
            .get_func(&mut store, func_extract_emails_export)
            .unwrap();
        let func_extract_emails_typed = instance
            .get_typed_func::<(String,), (Vec<String>,)>(&mut store, func_extract_emails_export)
            .unwrap();

        let emails0 = {
            let inputs = &[Val::String(
                "Hello my name is John Doe, my email is john.doe@gmail.com
                 I also have another email: john.doe@icloud.com
                 My friend's email is jane.doe@hotmail.com"
                    .to_owned(),
            )];
            let outputs = &mut [Val::Bool(false)];

            func_extract_emails
                .call(&mut store, inputs, outputs)
                .unwrap();
            func_extract_emails.post_return(&mut store).unwrap();

            let Val::List(l) = &outputs.get(0).unwrap() else {
                panic!("unexpected type")
            };

            l.iter()
                .map(|v| {
                    let Val::String(s) = v else {
                        panic!("unexpected type")
                    };
                    s.to_string()
                })
                .collect::<Vec<String>>()
        };

        let (emails1,) = func_extract_emails_typed
            .call(
                &mut store,
                ("Hello my name is John Doe, my email is john.doe@gmail.com
              I also have another email: john.doe@icloud.com
              My friend's email is jane.doe@hotmail.com"
                    .to_owned(),),
            )
            .unwrap();
        func_extract_emails_typed.post_return(&mut store).unwrap();

        assert_eq!(emails0, emails1);

        // Test Load Image

        let func_load_image_export = instance
            .get_export(&mut store, Some(&intf_export), "load-image")
            .unwrap();
        let func_load_image_typed = instance
            .get_typed_func::<(Vec<u8>,), (ResourceAny,)>(&mut store, func_load_image_export)
            .unwrap();

        let (img,) = func_load_image_typed
            .call(&mut store, (IMAGE.to_vec(),))
            .unwrap();
        func_load_image_typed.post_return(&mut store).unwrap();

        // Test Resize Image

        let func_resize_image_export = instance
            .get_export(&mut store, Some(&intf_export), "resize-image")
            .unwrap();
        let func_resize_image_typed = instance
            .get_typed_func::<(ResourceAny, u32, u32), (ResourceAny,)>(
                &mut store,
                func_resize_image_export,
            )
            .unwrap();

        let (img,) = func_resize_image_typed
            .call(&mut store, (img, 100, 100))
            .unwrap();
        func_resize_image_typed.post_return(&mut store).unwrap();

        // Test Image to Bytes

        let func_image_to_bytes_export = instance
            .get_export(&mut store, Some(&intf_export), "image-to-bytes")
            .unwrap();
        let func_image_to_bytes_typed = instance
            .get_typed_func::<(ResourceAny,), (Vec<u8>,)>(&mut store, func_image_to_bytes_export)
            .unwrap();

        let (bytes,) = func_image_to_bytes_typed.call(&mut store, (img,)).unwrap();
        func_image_to_bytes_typed.post_return(&mut store).unwrap();

        std::fs::write(
            concat!(env!("CARGO_MANIFEST_DIR"), "/cat_resized.png"),
            bytes,
        )
        .unwrap();

        println!("Image resized and saved to cat_resized.png");

        Ok(())
    }

    #[test]
    fn test_py_guest() -> anyhow::Result<()> {
        let config = Config::new();
        let engine = Engine::new(&config).unwrap();
        let host = Host::new();
        let mut store = Store::new(&engine, host);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync::<Host>(&mut linker).unwrap();

        let component = Component::from_binary(&engine, &GUEST_PY_WASI_COMPONENT).unwrap();
        let instance = linker.instantiate(&mut store, &component).unwrap();
        let f1 = instance
            .get_typed_func::<(String,), ()>(&mut store, "print")
            .unwrap();
        let () = f1.call(&mut store, ("Hello world".to_string(),)).unwrap();
        f1.post_return(&mut store).unwrap();
        Ok(())
    }

    #[test]
    fn test_js_guest() -> anyhow::Result<()> {
        let config = Config::new();
        let engine = Engine::new(&config).unwrap();
        let host = Host::new();
        let mut store = Store::new(&engine, host);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync::<Host>(&mut linker).unwrap();

        let component = Component::from_binary(&engine, &GUEST_JS_WASI_COMPONENT).unwrap();
        let instance = linker.instantiate(&mut store, &component).unwrap();
        let f1 = instance
            .get_typed_func::<(String,), ()>(&mut store, "print")
            .unwrap();
        let () = f1.call(&mut store, ("Hello world".to_string(),)).unwrap();
        Ok(())
    }
}
