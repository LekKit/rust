use crate::spec::{Os, RelroLevel, StackProbeType, TargetOptions, TlsModel, cvs};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: Os::Haiku,
        families: cvs!["unix"],
        has_rpath: true,
        dynamic_linking: true,
        plt_by_default: false,
        has_thread_local: true,
        position_independent_executables: true,
        tls_model: TlsModel::GeneralDynamic,
        relro_level: RelroLevel::Full,
        stack_probes: StackProbeType::Inline,
        ..Default::default()
    }
}
