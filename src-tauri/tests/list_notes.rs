use merino_lib::features::notes::{application::get::{get_notes_use_case, list_notes_use_case}, domain::{errors::NoteError, note::Note}};

mod mocks;
use mocks::notes_mock::MockNoteRepository;

#[tokio::test]
/// Test for "Happy path" where the space exists and contains notes.
async fn list_notes_returns_correct_number_of_notes() {
    let mock_repo = MockNoteRepository {
        notes: vec![
            Note {
                name: "Note 1".to_string()
            },
            Note {
                name: "Note 2".to_string()
            }
        ],
        should_error: false
    };
    let space_name = "test_space";
    let result = get_notes_use_case(&mock_repo, space_name).await;

    assert!(result.is_ok());
    let notes = result.unwrap();
    assert_eq!(notes.len(), 2);
    // You can't assert on the ID as is randomly generated, we just going to assert the name
    assert_eq!(notes[0].name, "Note 1");
}

#[tokio::test]
async fn list_notes_returns_empty_vec_for_empty_space() {
    let mock_repo = MockNoteRepository {
        notes: Vec::new(),
        should_error: false,
    };
    let space_name = "empty_space";
    let result = list_notes_use_case(&mock_repo, space_name).await;

    assert!(result.is_ok());
    let notes = result.unwrap();
    assert!(notes.is_empty());
}

#[tokio::test]
async fn list_notes_returns_not_found_error_for_noexisting_space() {
    let mock_repo = MockNoteRepository {
        notes: Vec::new(),
        should_error: true,
    };
    let space_name = "noexisting_space";
    let result = list_notes_use_case(&mock_repo, space_name).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(matches!(error, NoteError::NotFound(_)));
}