use crate::utils::{
    get_project_code::get_prj_code,
    get_project_version::get_version,
    get_project_name::get_project_name,
    get_filename::get_file_name,
};

use crate::helpers::{
    datetime::current_date,
    datetime::replace_date,
    split_string::split_file_name,
    split_string::join_part_file_name,
    rename::rename_file,
};

use crate::constants::{PRJ_CODE_POS, PRJ_NAME_POS, VERSION_POS};

fn apply_rename<F>(transform: F)
where
    F: Fn(&mut Vec<String>),
{
    let file_list: Vec<String> = get_file_name();

    if file_list.len() < 5 {
        return;
    }

    let rename_tasks: Vec<(String, String)> = file_list
        .into_iter()
        .map(|old_name: String| {
            let mut parts: Vec<String> = split_file_name(old_name.clone());
            transform(&mut parts);
            let new_name = join_part_file_name(&parts);
            (old_name, new_name)
        })
        .collect();

    rename_file(rename_tasks);
}

pub fn rename_prj_code() {
    let new_code: String = get_prj_code();
    apply_rename(|parts: &mut Vec<String>| {
        parts[PRJ_CODE_POS].clone_from(&new_code);
    });
}

pub fn rename_version() {
    let new_version: String = get_version();
    apply_rename(|parts: &mut Vec<String>| {
        parts[VERSION_POS].clone_from(&new_version);
    });
}

pub fn rename_prj_name() {
    let new_name: String = get_project_name();
    apply_rename(|parts: &mut Vec<String>| {
        parts[PRJ_NAME_POS].clone_from(&new_name);
    });
}

pub fn rename_date() {
    let new_date: String = current_date();
    apply_rename(|parts: &mut Vec<String>| {
        replace_date(parts, &new_date);
    });
}

pub fn new_project() {
    let new_code: String    = get_prj_code();
    let new_version: String = get_version();
    let new_name: String    = get_project_name();
    let new_date: String    = current_date();

    apply_rename(|parts: &mut Vec<String>| {
        parts[PRJ_CODE_POS].clone_from(&new_code);
        parts[VERSION_POS].clone_from(&new_version);
        parts[PRJ_NAME_POS].clone_from(&new_name);
        replace_date(parts, &new_date);
    });
}