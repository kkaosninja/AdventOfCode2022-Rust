use std::fs::read_to_string;

use log::{debug, trace};

const INPUT_FILENAME: &str = "puzzle.txt";

fn main() {
    env_logger::init();
    debug!("Starting Application!");

    let input_file_lines: Vec<String> = read_to_string(INPUT_FILENAME)
        .expect("Could not read input file!")
        .split('\n')
        .map(|line| String::from(line))
        .collect();

    // Part 1 answer
    // All length-wise edge trees + all width-wise edge trees.
    // Subtract four from this to ensure the four corner trees are not counted twice

    let mut input_data_matrix: Vec<Vec<usize>> = Vec::new();

    process_input(&mut input_data_matrix, input_file_lines);
    trace!("Input Data Matrix: {:?}", input_data_matrix);

    //Immutable to mutable
    let input_data_matrix = input_data_matrix;

    /*
    ALright, let's go for a O(n*m) solution, instead of O(n^2 * m^2) brute-force solution
    where n = no. of lines and m = no. of chars on each line
    We will accomplish this by making passes on this data, and systematically
    setting the visibility value of each tree to true in the visibility matrix
    */

    // The visibility matrix is a matrix of boolean values that is of the same dimensions as the actual data
    // A value of true represents visible, and vice versa.
    let mut visibility_bool_matrix: Vec<Vec<bool>> = Vec::new();
    populate_visibility_matrix(&input_data_matrix, &mut visibility_bool_matrix);
    trace!("Tree Visibility Matrix: {:?}", visibility_bool_matrix);

    /* Let's call our algorithm tallest-tree-so-far.
        For each tree in a row/column, in all orientations aka
        left-to-right + right-to-left and top-to-bottom + bottom-to-top
        if its height is the tallest we've seen so far, we set its visibility to true.
        We stop traversing if we come across a height that is the maximum height for the row
        Obviously the first occurance of  a tree with this max height will be visible, but not anything after that

        So overall, for each tree, we do four passes. If its visibility gets set to true even in one pass,
        it means its visible from somewhere, which is enough

        This is implemented in set_visibility_row_wise() and set_visibility_column_wise()
        which we will call on each row and column respectively
        and appropriately set the values in the visibility matrix

        When this is done, we have a visibility matrix ready.
    */

    //Compute visibility row-wose
    for row_index in 0..input_data_matrix.len() {
        set_visibility_row_wise(&input_data_matrix, &mut visibility_bool_matrix, row_index);
        trace!("--");
    }

    for column_index in 0..input_data_matrix[0].len() {
        set_visibility_column_wise(
            &input_data_matrix,
            &mut visibility_bool_matrix,
            column_index,
        );
        trace!("--");
    }
    debug!("Printing Visibility matrix!");
    for row_index in 0..visibility_bool_matrix.len() {
        debug!("{:?}", visibility_bool_matrix[row_index]);
    }

    //Count visible trees
    let mut visible_trees = 0;
    for row_index in 0..visibility_bool_matrix.len() {
        for column_index in 0..visibility_bool_matrix[row_index].len() {
            if visibility_bool_matrix[row_index][column_index] {
                visible_trees += 1;
            }
        }
    }
    println!(
        "Part 1 | How many trees are visible from outside the grid?\nAnswer: {}",
        visible_trees
    );

    // Solve for Part 2

    /*
    We are going for a brute-force solution here
    We can do two optimizations -
    1) Don't calculate scenic scores for edge trees - as it will be zero
    2) Don't calculate scenic scores for interior trees that are not visible - not likely to have a very high score.

    Visible trees will have higher scenic scores than invisible trees

    */

    // let test_score = compute_scenic_score(&input_data_matrix, &visibility_bool_matrix, 3, 2);
    // debug!("Test score for tree at 3,2 is {}",test_score);

    let mut max_scenic_score = 0;
    for row_index in 0..visibility_bool_matrix.len() {
        for column_index in 0..visibility_bool_matrix[row_index].len() {
            let current_scenic_score = compute_scenic_score(
                &input_data_matrix,
                &visibility_bool_matrix,
                row_index,
                column_index,
            );
            if current_scenic_score > max_scenic_score {
                max_scenic_score = current_scenic_score;
            }
        }
    }
    println!(
        "Part 2 | What is the highest scenic score possible for any tree?\nAnswer: {}",
        max_scenic_score
    );
}

fn process_input(input_data_array: &mut Vec<Vec<usize>>, input_file_lines: Vec<String>) {
    // For each input line, populate the input_data_vector

    for input_file_line in input_file_lines {
        let mut line_data_vector: Vec<usize> = Vec::new();

        // Call trim() to ensure whitespace on the ends are removed
        for tree_height_str in input_file_line.trim().chars().map(|c| c.to_string()) {
            // trace!("Parsing character height {}", tree_height_str);
            let tree_height = tree_height_str
                .parse::<usize>()
                .expect("Could not parse tree height integer");
            line_data_vector.push(tree_height);
        }

        input_data_array.push(line_data_vector);
    }
}

fn populate_visibility_matrix(
    input_data_matrix: &Vec<Vec<usize>>,
    visibility_bool_matrix: &mut Vec<Vec<bool>>,
) {
    for input_data_row in input_data_matrix {
        let mut visibility_matrix_row: Vec<bool> = Vec::new();

        for _ in input_data_row {
            visibility_matrix_row.push(false);
        }

        visibility_bool_matrix.push(visibility_matrix_row);
    }
}

fn set_visibility_row_wise(
    input_data_matrix: &Vec<Vec<usize>>,
    visibility_bool_matrix: &mut Vec<Vec<bool>>,
    row_index: usize,
) {
    //Get the data for the row which we are processing
    let current_data_row = input_data_matrix
        .get(row_index)
        .expect("Could not fetch row given row number");

    let mut row_max_tree_height: usize = 0;

    current_data_row.iter().for_each(|&tree_height| {
        if (row_max_tree_height == 0) | (tree_height > row_max_tree_height) {
            row_max_tree_height = tree_height;
        }
    });

    // No longer mutable
    let row_max_tree_height = row_max_tree_height;

    debug!(
        "Max Tree height for row {} is {}",
        row_index, row_max_tree_height
    );

    // Set visibility of edge trees to true, as per puzzle rules
    visibility_bool_matrix[row_index][0] = true;
    visibility_bool_matrix[row_index][current_data_row.len() - 1] = true;

    // Time to set the visibility of the trees

    // Part 1 - Left to right
    let mut tallest_tree_so_far = current_data_row[0];

    // Only traverse over internal trees.
    for column_index in 1..(current_data_row.len() - 1) {
        let current_tree_height = current_data_row[column_index];
        trace!(
            "Current tree height at row {} column {} is {}",
            row_index,
            column_index,
            current_tree_height
        );

        // We've encountered a tree even taller than the tallest one so far. Its definitely visible
        // Also change tallest_tree_so_far value to reflect this
        if current_tree_height > tallest_tree_so_far {
            visibility_bool_matrix[row_index][column_index] = true;
            tallest_tree_so_far = current_tree_height;
        }

        // If this tree height is the maximum height for this row,
        // there's no point going forward
        if current_tree_height == row_max_tree_height {
            trace!(
                "Encountered max tree height at row {} colunm {}. Breaking loop",
                row_index,
                column_index
            );
            break;
        }
    } // for loop

    trace!("Right to left now!");

    // Part 2 - Right to left
    let mut tallest_tree_so_far = current_data_row[current_data_row.len() - 1];

    // New feature learned. Reverse Iterator!
    for column_index in (1..(current_data_row.len() - 1)).rev() {
        let current_tree_height = current_data_row[column_index];
        trace!(
            "Current tree height at row {} column {} is {}",
            row_index,
            column_index,
            current_tree_height
        );

        if current_tree_height > tallest_tree_so_far {
            visibility_bool_matrix[row_index][column_index] = true;
            tallest_tree_so_far = current_tree_height;
        }

        // If this tree height is the maximum height for this row,
        // there's no point going forward
        if current_tree_height == row_max_tree_height {
            trace!(
                "Encountered max tree height at row {} colunm {}. Breaking loop",
                row_index,
                column_index
            );
            break;
        }
    }
}

fn set_visibility_column_wise(
    input_data_matrix: &Vec<Vec<usize>>,
    visibility_bool_matrix: &mut Vec<Vec<bool>>,
    column_index: usize,
) {
    //We will have to do this slightly differently

    // Find the max height for this column
    let mut column_max_tree_height = 0;

    for row_index in 0..input_data_matrix.len() {
        let current_tree_height = input_data_matrix[row_index][column_index];

        if (column_max_tree_height == 0) | (current_tree_height > column_max_tree_height) {
            column_max_tree_height = current_tree_height;
        }
    }

    debug!(
        "Max tree height for column {} is {}",
        column_index, column_max_tree_height
    );

    //Set edge trees visibility to true
    visibility_bool_matrix[0][column_index] = true;
    visibility_bool_matrix[input_data_matrix.len() - 1][column_index] = true;

    // Time to set the visibilty of the trees

    // Part 1 - Top to Bottom
    let mut tallest_tree_so_far = input_data_matrix[0][column_index];

    for row_index in 1..(input_data_matrix.len() - 1) {
        let current_tree_height = input_data_matrix[row_index][column_index];
        trace!(
            "Current tree height at row {} column {} is {}",
            row_index,
            column_index,
            current_tree_height
        );

        // We've encountered a tree even taller than the tallest one so far. Its definitely visible
        // Also change tallest_tree_so_far value to reflect this
        if current_tree_height > tallest_tree_so_far {
            visibility_bool_matrix[row_index][column_index] = true;
            tallest_tree_so_far = current_tree_height;
        }

        // If this tree height is the maximum height for this column,
        // there's no point going forward
        if current_tree_height == column_max_tree_height {
            trace!(
                "Encountered max tree height at row {} colunm {}. Breaking loop",
                row_index,
                column_index
            );
            break;
        }
    } //for loop

    // Part 2 - Bottom to top
    trace!("Bottom to top now!");
    let mut tallest_tree_so_far = input_data_matrix[input_data_matrix.len() - 1][column_index];

    for row_index in (1..(input_data_matrix.len() - 1)).rev() {
        let current_tree_height = input_data_matrix[row_index][column_index];
        trace!(
            "Current tree height at row {} column {} is {}",
            row_index,
            column_index,
            current_tree_height
        );

        // We've encountered a tree even taller than the tallest one so far. Its definitely visible
        // Also change tallest_tree_so_far value to reflect this
        if current_tree_height > tallest_tree_so_far {
            visibility_bool_matrix[row_index][column_index] = true;
            tallest_tree_so_far = current_tree_height;
        }

        // If this tree height is the maximum height for this column,
        // there's no point going forward
        if current_tree_height == column_max_tree_height {
            trace!(
                "Encountered max tree height at row {} colunm {}. Breaking loop",
                row_index,
                column_index
            );
            break;
        }
    } //for loop
}

fn compute_scenic_score(
    input_data_matrix: &Vec<Vec<usize>>,
    visibility_bool_matrix: &Vec<Vec<bool>>,
    row_index: usize,
    column_index: usize,
) -> usize {
    let mut scenic_score_for_this_tree: usize = 1;

    // If this is an edge tree, then its scenic score is zero. Return immediately
    if (row_index == 0)
        | (column_index == 0)
        | (row_index == input_data_matrix.len() - 1)
        | (column_index == input_data_matrix[0].len() - 1)
    {
        return 0;
    }

    trace!(
        "Calculating scenic score for row {} col {}",
        row_index,
        column_index
    );

    //If this tree is not "visible" then we don't bother calculating
    if !visibility_bool_matrix[row_index][column_index] {
        return scenic_score_for_this_tree;
    }

    let scoring_tree_height = input_data_matrix[row_index][column_index];

    // Part 1 - Row-wise scenic score

    // Part 1-1 - Check row-wise visibility to the left of the tree
    let mut visible_trees_left = 0;

    for current_column_index in (0..=(column_index - 1)).rev() {
        // If we got to here, then the tree in this column is visible
        visible_trees_left += 1;

        // This is the msot we can see. No need to go further
        if scoring_tree_height <= input_data_matrix[row_index][current_column_index] {
            break;
        }
    }
    trace!(
        "Visible trees to left for tree at row {} col {} is {}",
        row_index,
        column_index,
        visible_trees_left
    );

    // Update scenic score
    scenic_score_for_this_tree *= visible_trees_left;

    // Part 1-2 - Check row-wise visibility to the right of the tree
    let mut visible_trees_right = 0;
    for current_column_index in (column_index + 1)..input_data_matrix[0].len() {
        visible_trees_right += 1;

        if scoring_tree_height <= input_data_matrix[row_index][current_column_index] {
            break;
        }
    }
    trace!(
        "Visible trees to right for tree at row {} col {} is {}",
        row_index,
        column_index,
        visible_trees_right
    );
    scenic_score_for_this_tree *= visible_trees_right;

    // Part 2 - Column-wise scenic score

    // Part 2-1 - Check column-wise visibility above current tree
    let mut visible_trees_top = 0;
    for current_row_index in (0..=(row_index - 1)).rev() {
        visible_trees_top += 1;

        if scoring_tree_height <= input_data_matrix[current_row_index][column_index] {
            break;
        }
    }
    trace!(
        "Visible trees to top for tree at row {} col {} is {}",
        row_index,
        column_index,
        visible_trees_top
    );
    scenic_score_for_this_tree *= visible_trees_top;

    // Part 2-2 - Check column-wise visibility below current tree
    let mut visible_trees_below = 0;
    for current_row_index in (row_index + 1)..input_data_matrix.len() {
        visible_trees_below += 1;
        if scoring_tree_height <= input_data_matrix[current_row_index][column_index] {
            break;
        }
    }
    trace!(
        "Visible trees to bottom for tree at row {} col {} is {}",
        row_index,
        column_index,
        visible_trees_below
    );
    scenic_score_for_this_tree *= visible_trees_below;

    return scenic_score_for_this_tree;
}
