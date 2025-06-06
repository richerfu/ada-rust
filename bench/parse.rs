use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};

const URLS: &[&str] = &[
    "https://www.google.com/webhp?hl=en&amp;ictx=2&amp;sa=X&amp;ved=0ahUKEwil_oSxzJj8AhVtEFkFHTHnCGQQPQgI",
    "https://support.google.com/websearch/?p=ws_results_help&amp;hl=en-CA&amp;fg=1",
    "https://en.wikipedia.org/wiki/Dog#Roles_with_humans",
    "https://www.tiktok.com/@aguyandagolden/video/7133277734310038830",
    "https://business.twitter.com/en/help/troubleshooting/how-twitter-ads-work.html?ref=web-twc-ao-gbl-adsinfo&utm_source=twc&utm_medium=web&utm_campaign=ao&utm_content=adsinfo",
    "https://images-na.ssl-images-amazon.com/images/I/41Gc3C8UysL.css?AUIClients/AmazonGatewayAuiAssets",
    "https://www.reddit.com/?after=t3_zvz1ze",
    "https://www.reddit.com/login/?dest=https%3A%2F%2Fwww.reddit.com%2F",
    "postgresql://other:9818274x1!!@localhost:5432/otherdb?connect_timeout=10&application_name=myapp",
    "http://192.168.1.1",            // ipv4
    "http://[2606:4700:4700::1111]", // ipv6
];

pub fn parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");
    group.throughput(Throughput::Bytes(URLS.iter().map(|u| u.len() as u64).sum()));
    group.bench_function("ada_url", |b| {
        b.iter(|| {
            URLS.iter().for_each(|url| {
                ada_url::Url::try_from(*black_box(url)).unwrap();
            })
        })
    });
    group.bench_function("url", |b| {
        b.iter(|| {
            URLS.iter().for_each(|url| {
                black_box(url).parse::<url::Url>().unwrap();
            })
        })
    });
    group.finish();
}

pub fn can_parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("can_parse");
    group.throughput(Throughput::Bytes(URLS.iter().map(|u| u.len() as u64).sum()));
    group.bench_function("ada_url", |b| {
        b.iter(|| {
            URLS.iter().for_each(|url| {
                let _ = ada_url::Url::can_parse(*black_box(url), None);
            })
        })
    });
    // TODO: Add `url` crate when it supports can_parse function.
    group.finish();
}

criterion_group!(benches, parse_benchmark, can_parse_benchmark);
criterion_main!(benches);
