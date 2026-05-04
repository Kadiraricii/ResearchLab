use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vercel_hack_analysis::analyzer::{run_all_analyzers, env_exposure, headers, cors, source_maps};
use vercel_hack_analysis::remediation::generate_report;

const VULNERABLE_VERCEL_JSON: &str = r#"{
    "redirects": [{"source":"/:path*","destination":"http://external.com/:path*","permanent":false}],
    "buildCommand": "npm run build -- --secret=test_value"
}"#;

const VULNERABLE_NEXT_CONFIG: &str = "productionBrowserSourceMaps: true";

const VULNERABLE_ENV: &str = "NEXT_PUBLIC_API_KEY=sk-supersecret\nNEXT_PUBLIC_SECRET_TOKEN=abc123\nNEXT_PUBLIC_AUTH_KEY=xyz789";

fn bench_env_exposure(c: &mut Criterion) {
    c.bench_function("env_exposure::analyze (10 vars)", |b| {
        let env = "NEXT_PUBLIC_SECRET_KEY=test\nNEXT_PUBLIC_API_TOKEN=test2\nNEXT_PUBLIC_AUTH=test3\nNEXT_PUBLIC_PASSWORD=test4\nDATABASE_URL=postgres://localhost\nNEXT_PUBLIC_NAME=App\nAPP_VERSION=1.0\nNEXT_PUBLIC_KEY=k1\nNEXT_PUBLIC_CREDENTIAL=c1\nNEXT_PUBLIC_SIGNING=s1";
        b.iter(|| env_exposure::analyze(black_box(env)))
    });
}

fn bench_header_analysis(c: &mut Criterion) {
    c.bench_function("headers::analyze", |b| {
        b.iter(|| headers::analyze(black_box(VULNERABLE_VERCEL_JSON)))
    });
}

fn bench_cors_analysis(c: &mut Criterion) {
    c.bench_function("cors::analyze", |b| {
        b.iter(|| cors::analyze(black_box(VULNERABLE_VERCEL_JSON), black_box(VULNERABLE_NEXT_CONFIG)))
    });
}

fn bench_source_maps(c: &mut Criterion) {
    c.bench_function("source_maps::analyze", |b| {
        b.iter(|| source_maps::analyze(black_box(VULNERABLE_NEXT_CONFIG)))
    });
}

fn bench_full_scan(c: &mut Criterion) {
    c.bench_function("run_all_analyzers (full scan)", |b| {
        b.iter(|| {
            run_all_analyzers(
                black_box(VULNERABLE_VERCEL_JSON),
                black_box(VULNERABLE_NEXT_CONFIG),
                black_box(VULNERABLE_ENV),
            )
        })
    });
}

fn bench_remediation_report(c: &mut Criterion) {
    let report = run_all_analyzers(VULNERABLE_VERCEL_JSON, VULNERABLE_NEXT_CONFIG, VULNERABLE_ENV);
    c.bench_function("generate_report (remediation)", |b| {
        b.iter(|| generate_report(black_box(&report)))
    });
}

criterion_group!(
    benches,
    bench_env_exposure,
    bench_header_analysis,
    bench_cors_analysis,
    bench_source_maps,
    bench_full_scan,
    bench_remediation_report,
);
criterion_main!(benches);
