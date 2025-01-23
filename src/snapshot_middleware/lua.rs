use std::{collections::HashMap, path::Path, str};

use memofs::{IoResultExt, Vfs};
use rbx_dom_weak::types::Enum;

use crate::snapshot::{InstanceContext, InstanceMetadata, InstanceSnapshot};

use super::meta_file::AdjacentMetadata;

#[derive(Debug)]
pub enum ScriptType {
    Server,
    Client,
    Module,
}

/// Core routine for turning Lua files into snapshots.
pub fn snapshot_lua(
    context: &InstanceContext,
    vfs: &Vfs,
    path: &Path,
    name: &str,
    script_type: ScriptType,
) -> anyhow::Result<Option<InstanceSnapshot>> {
    let run_context_enums = &rbx_reflection_database::get()
        .enums
        .get("RunContext")
        .expect("Unable to get RunContext enums!")
        .items;

    let (class_name, run_context) = match (context.emit_legacy_scripts, script_type) {
        (false, ScriptType::Server) => ("Script", run_context_enums.get("Server")),
        (false, ScriptType::Client) => ("Script", run_context_enums.get("Client")),
        (true, ScriptType::Server) => ("Script", run_context_enums.get("Legacy")),
        (true, ScriptType::Client) => ("LocalScript", None),
        (_, ScriptType::Module) => ("ModuleScript", None),
    };

    let contents = vfs.read_to_string_lf_normalized(path)?;
    let contents_str = contents.as_str();

    let mut properties = HashMap::with_capacity(2);
    properties.insert("Source".to_owned(), contents_str.into());

    if let Some(run_context) = run_context {
        properties.insert(
            "RunContext".to_owned(),
            Enum::from_u32(run_context.to_owned()).into(),
        );
    }

    let meta_path = path.with_file_name(format!("{}.meta.json", name));

    let mut snapshot = InstanceSnapshot::new()
        .name(name)
        .class_name(class_name)
        .properties(properties)
        .metadata(
            InstanceMetadata::new()
                .instigating_source(path)
                .relevant_paths(vec![path.to_path_buf(), meta_path.clone()])
                .context(context),
        );

    if let Some(meta_contents) = vfs.read(&meta_path).with_not_found()? {
        let mut metadata = AdjacentMetadata::from_slice(&meta_contents, meta_path)?;
        metadata.apply_all(&mut snapshot)?;
    }

    Ok(Some(snapshot))
}

#[cfg(test)]
mod test {
    use super::*;

    use memofs::{InMemoryFs, VfsSnapshot};

    #[test]
    fn class_module_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/foo.lua"),
            "foo",
            ScriptType::Module,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn runcontext_module_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(false)),
            &vfs,
            Path::new("/foo.lua"),
            "foo",
            ScriptType::Module,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn class_server_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.server.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/foo.server.lua"),
            "foo",
            ScriptType::Server,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn runcontext_server_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.server.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(false)),
            &vfs,
            Path::new("/foo.server.lua"),
            "foo",
            ScriptType::Server,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn class_client_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.client.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/foo.client.lua"),
            "foo",
            ScriptType::Client,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn runcontext_client_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.client.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(false)),
            &vfs,
            Path::new("/foo.client.lua"),
            "foo",
            ScriptType::Client,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[ignore = "init.lua functionality has moved to the root snapshot function"]
    #[test]
    fn init_module_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot(
            "/root",
            VfsSnapshot::dir([("init.lua", VfsSnapshot::file("Hello!"))]),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/root"),
            "root",
            ScriptType::Module,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn class_module_with_meta() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();
        imfs.load_snapshot(
            "/foo.meta.json",
            VfsSnapshot::file(
                r#"
                    {
                        "ignoreUnknownInstances": true
                    }
                "#,
            ),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/foo.lua"),
            "foo",
            ScriptType::Module,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn runcontext_module_with_meta() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();
        imfs.load_snapshot(
            "/foo.meta.json",
            VfsSnapshot::file(
                r#"
                    {
                        "ignoreUnknownInstances": true
                    }
                "#,
            ),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(false)),
            &vfs,
            Path::new("/foo.lua"),
            "foo",
            ScriptType::Module,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn class_script_with_meta() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.server.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();
        imfs.load_snapshot(
            "/foo.meta.json",
            VfsSnapshot::file(
                r#"
                    {
                        "ignoreUnknownInstances": true
                    }
                "#,
            ),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/foo.server.lua"),
            "foo",
            ScriptType::Server,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn runcontext_script_with_meta() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/foo.server.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();
        imfs.load_snapshot(
            "/foo.meta.json",
            VfsSnapshot::file(
                r#"
                    {
                        "ignoreUnknownInstances": true
                    }
                "#,
            ),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(false)),
            &vfs,
            Path::new("/foo.server.lua"),
            "foo",
            ScriptType::Server,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn class_script_disabled() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/bar.server.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();
        imfs.load_snapshot(
            "/bar.meta.json",
            VfsSnapshot::file(
                r#"
                    {
                        "properties": {
                            "Disabled": true
                        }
                    }
                "#,
            ),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(true)),
            &vfs,
            Path::new("/bar.server.lua"),
            "bar",
            ScriptType::Server,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }

    #[test]
    fn runcontext_script_disabled() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot("/bar.server.lua", VfsSnapshot::file("Hello there!"))
            .unwrap();
        imfs.load_snapshot(
            "/bar.meta.json",
            VfsSnapshot::file(
                r#"
                    {
                        "properties": {
                            "Disabled": true
                        }
                    }
                "#,
            ),
        )
        .unwrap();

        let vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_lua(
            &InstanceContext::with_emit_legacy_scripts(Some(false)),
            &vfs,
            Path::new("/bar.server.lua"),
            "bar",
            ScriptType::Server,
        )
        .unwrap()
        .unwrap();

        insta::with_settings!({ sort_maps => true }, {
            insta::assert_yaml_snapshot!(instance_snapshot);
        });
    }
}
