pub fn write_sc_as(calls: &[String]) -> String {
    format!(
        "import {{env}} from '../../as/env';
        import {{ toBytes, fromBytes }} from '../../as/helpers';

export function main(): void {{
{}
}}",
        calls.join("\n")
    )
}
