use super::*;

#[test]
fn test_with_duplicate_column_empty_df() {
    let a = Int32Chunked::from_slice("a", &[]);

    assert_eq!(
        DataFrame::new(vec![a.into_series()])
            .unwrap()
            .lazy()
            .with_columns([lit(true).alias("a")])
            .collect()
            .unwrap()
            .get_column_names(),
        &["a"]
    );
}

#[test]
fn test_drop() -> Result<()> {
    // dropping all columns is a special case. It may fail because a projection
    // that projects nothing could be misinterpreted as select all.
    let out = df![
        "a" => [1],
    ]?
    .lazy()
    .drop_columns(["a"])
    .collect()?;
    assert_eq!(out.width(), 0);
    Ok(())
}

#[test]
#[cfg(feature = "dynamic_groupby")]
fn test_special_groupby_schemas() -> Result<()> {
    let df = df![
        "a" => [1, 2, 3, 4, 5],
        "b" => [1, 2, 3, 4, 5],
    ]?;

    let out = df
        .clone()
        .lazy()
        .groupby_rolling(
            [],
            RollingGroupOptions {
                index_column: "a".into(),
                period: Duration::parse("2i"),
                offset: Duration::parse("0i"),
                closed_window: ClosedWindow::Left,
            },
        )
        .agg([col("b").sum().alias("sum")])
        .select([col("a"), col("sum")])
        .collect()?;

    assert_eq!(
        out.column("sum")?
            .i32()?
            .into_no_null_iter()
            .collect::<Vec<_>>(),
        &[3, 5, 7, 9, 5]
    );

    let out = df
        .lazy()
        .groupby_dynamic(
            [],
            DynamicGroupOptions {
                index_column: "a".into(),
                every: Duration::parse("2i"),
                period: Duration::parse("2i"),
                offset: Duration::parse("0i"),
                truncate: false,
                include_boundaries: false,
                closed_window: ClosedWindow::Left,
            },
        )
        .agg([col("b").sum().alias("sum")])
        .select([col("a"), col("sum")])
        .collect()?;

    assert_eq!(
        out.column("sum")?
            .i32()?
            .into_no_null_iter()
            .collect::<Vec<_>>(),
        &[1, 5, 9]
    );

    Ok(())
}
