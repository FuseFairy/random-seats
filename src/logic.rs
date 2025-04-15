use anyhow::{Context, Result};
use rand::rng;
use rand::seq::SliceRandom;
use std::path::Path;
use umya_spreadsheet::{
    Alignment, Font, HorizontalAlignmentValues, Style, Worksheet, reader, writer,
};

pub fn read_student(student_xlsx_path: &str) -> Result<Vec<String>> {
    let path = Path::new(student_xlsx_path);
    let book = reader::xlsx::read(path)
        .with_context(|| format!("無法讀取 Excel 檔案：{}", student_xlsx_path))?;

    let sheet: &Worksheet = book
        .get_sheet_by_name("student")
        .context("找不到 student 工作表")?;

    let (_, max_row) = sheet.get_highest_column_and_row();
    let mut names = Vec::new();

    for row_idx in 1..=max_row {
        let cell_ref = format!("A{}", row_idx);
        if let Some(cell) = sheet.get_cell(cell_ref.as_str()) {
            names.push(cell.get_value().to_string());
        }
    }

    Ok(names)
}

pub fn assign_students_to_tags(
    students: &[String],
    max_seats: usize,
    excluded_seats: &[String],
) -> Result<Vec<(String, String)>> {
    if students.len() > max_seats - excluded_seats.len() {
        return Err(anyhow::anyhow!("學生人數超過可分配的座位數量！"));
    }

    let mut rng = rng();
    let mut shuffled_students = students.to_vec();
    shuffled_students.shuffle(&mut rng);

    let all_tags: Vec<String> = (1..=max_seats).map(|i| format!("A{:03}", i)).collect();
    let excluded_set: std::collections::HashSet<String> = excluded_seats.iter().cloned().collect();

    let available_tags: Vec<String> = all_tags
        .iter()
        .filter(|tag| !excluded_set.contains(*tag))
        .cloned()
        .collect();

    let mut assignments: Vec<(String, String)> = available_tags
        .into_iter()
        .zip(shuffled_students.into_iter())
        .collect();

    let filled_tags: std::collections::HashSet<String> =
        assignments.iter().map(|(t, _)| t.clone()).collect();
    for tag in all_tags.iter() {
        if !filled_tags.contains(tag) {
            assignments.push((tag.clone(), String::new()));
        }
    }

    Ok(assignments)
}

pub fn write_seats_to_excel(
    student_xlsx_path: &str,
    assignments: &[(String, String)],
) -> Result<()> {
    let path = Path::new(student_xlsx_path);
    let mut book = reader::xlsx::read(path)
        .with_context(|| format!("無法打開 Excel 檔案：{}", student_xlsx_path))?;

    let sheet = book
        .get_sheet_by_name_mut("seats")
        .context("找不到 seats 工作表")?;

    let mut style = Style::default();
    let mut font = Font::default();
    font.set_name("新細明體");
    font.set_charset(136);
    font.set_size(14.0);
    font.set_bold(true);
    style.set_font(font);

    let mut alignment = Alignment::default();
    alignment.set_horizontal(HorizontalAlignmentValues::Center);
    style.set_alignment(alignment);

    for (seat, name) in assignments {
        let row_index = seat[1..]
            .parse::<u32>()
            .context(format!("無法解析座位標籤 {}", seat))?;
        let cell_coordinate = format!("A{}", row_index);

        let cell = sheet.get_cell_mut(cell_coordinate.as_str());
        cell.set_value(name);
        cell.set_style(style.clone());
    }

    writer::xlsx::write(&book, path)
        .with_context(|| format!("無法寫入 Excel 檔案：{}", student_xlsx_path))?;

    Ok(())
}
