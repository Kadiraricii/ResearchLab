use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vercel_hack_analysis::db::{crud, models, Database};

fn setup_db() -> Database {
    Database::open_in_memory().expect("in-memory DB")
}

fn bench_scan_insert(c: &mut Criterion) {
    let db = setup_db();
    let conn = db.conn.lock().unwrap();

    c.bench_function("crud::insert_scan", |b| {
        let mut i = 0u64;
        b.iter(|| {
            i += 1;
            let scan = models::Scan {
                id:            format!("scan-{i}"),
                created_at:    "2026-05-05T00:00:00Z".to_string(),
                score:         7.5,
                finding_count: 3,
                vercel_json:   Some("{}".to_string()),
                next_config:   None,
                env_content:   None,
            };
            crud::insert_scan(black_box(&conn), black_box(&scan)).unwrap();
        });
    });
}

fn bench_scan_list(c: &mut Criterion) {
    let db = setup_db();
    let conn = db.conn.lock().unwrap();

    // Seed 100 scans
    for i in 0..100 {
        let scan = models::Scan {
            id: format!("seed-{i}"),
            created_at: "2026-05-05T00:00:00Z".to_string(),
            score: 5.0,
            finding_count: 0,
            vercel_json: None,
            next_config: None,
            env_content: None,
        };
        crud::insert_scan(&conn, &scan).unwrap();
    }

    c.bench_function("crud::list_scans (100 rows)", |b| {
        b.iter(|| crud::list_scans(black_box(&conn)).unwrap());
    });
}

fn bench_findings_insert_batch(c: &mut Criterion) {
    let db = setup_db();
    let conn = db.conn.lock().unwrap();

    // Pre-insert a parent scan
    let scan = models::Scan {
        id: "parent".to_string(),
        created_at: "2026-05-05T00:00:00Z".to_string(),
        score: 8.0,
        finding_count: 12,
        vercel_json: None,
        next_config: None,
        env_content: None,
    };
    crud::insert_scan(&conn, &scan).unwrap();

    c.bench_function("crud::insert_findings (12 findings)", |b| {
        let mut batch_id = 0u64;
        b.iter(|| {
            batch_id += 1;
            let findings: Vec<models::Finding> = (0..12)
                .map(|j| models::Finding {
                    id:                 format!("f-{batch_id}-{j}"),
                    scan_id:            "parent".to_string(),
                    title:              format!("Finding {j}"),
                    description:        "desc".to_string(),
                    risk_level:         "High".to_string(),
                    affected_component: "headers".to_string(),
                    remediation:        "fix it".to_string(),
                })
                .collect();
            crud::insert_findings(black_box(&conn), black_box(&findings)).unwrap();
        });
    });
}

criterion_group!(benches, bench_scan_insert, bench_scan_list, bench_findings_insert_batch);
criterion_main!(benches);
