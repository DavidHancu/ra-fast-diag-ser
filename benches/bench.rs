use std::str::FromStr;

use criterion::BenchmarkId;

use criterion::{criterion_group, criterion_main, Criterion};
use lsp_types::{Diagnostic, DiagnosticRelatedInformation, Position, Range};
use ra_fast_diag_ser::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let diagnostic = Diagnostic {
        range: Range {
            start: lsp_types::Position {
                line: 1928,
                character: 28034,
            },
            end: lsp_types::Position {
                line: 28034,
                character: 1928,
            },
        },
        severity: Some(lsp_types::DiagnosticSeverity::ERROR),
        code: None,
        code_description: None,
        source: Some("rustc".to_string()),
        message: "messagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessage".to_string(),
        related_information: Some(vec![DiagnosticRelatedInformation {
            location: lsp_types::Location {
                uri: lsp_types::Uri::from_str("file:///path/to/file").unwrap(),
                range: Range {
                    start: Position {
                        line: 1928,
                        character: 28034,
                    },
                    end: Position {
                        line: 28034,
                        character: 1928,
                    },
                },
            },
            message: "messagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessage".to_string(),
        }, DiagnosticRelatedInformation {
            location: lsp_types::Location {
                uri: lsp_types::Uri::from_str("file:///path/to/file").unwrap(),
                range: Range {
                    start: Position {
                        line: 1928,
                        character: 28034,
                    },
                    end: Position {
                        line: 28034,
                        character: 1928,
                    },
                },
            },
            message: "messagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessage".to_string(),
        }, DiagnosticRelatedInformation {
            location: lsp_types::Location {
                uri: lsp_types::Uri::from_str("file:///path/to/file").unwrap(),
                range: Range {
                    start: Position {
                        line: 1928,
                        character: 28034,
                    },
                    end: Position {
                        line: 28034,
                        character: 1928,
                    },
                },
            },
            message: "messagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessagemessage".to_string(),
        }]),
        tags: None,
        data: None,
    };

    c.bench_with_input(BenchmarkId::new("serde_serialize_diagnostic", ""), &diagnostic, |b, diagnostic| {
        b.iter(|| serde_serialize_diagnostic(diagnostic));
    });
    c.bench_with_input(BenchmarkId::new("sonic_serialize_diagnostic", ""), &diagnostic, |b, diagnostic| {
        b.iter(|| sonic_serialize_diagnostic(diagnostic));
    });
    c.bench_with_input(BenchmarkId::new("fast_serialize_error_diagnostic", ""), &diagnostic, |b, diagnostic| {
        b.iter(|| fast_serialize_error_diagnostic(diagnostic));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);