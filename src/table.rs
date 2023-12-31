use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableError {
    InvalidNumberOfColumns,
    InvalidNumberOfColumnNames,
    InvalidNumberOfElements,
}

impl Error for TableError {}
impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "C3dParseError: {:?}", self)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Order {
    #[default]
    RowMajor,
    ColumnMajor,
}

impl Order {
    fn swap(&mut self) {
        *self = match *self {
            Order::RowMajor => Order::ColumnMajor,
            Order::ColumnMajor => Order::RowMajor,
        }
    }
}

pub struct Table<T> {
    col_names: Vec<String>,
    descriptions: Vec<String>,
    data: Vec<T>,
    num_cols: usize,
    num_rows: usize,
    order: Order,
}

impl<T> Table<T> where T: Default + Clone + Copy {
    pub fn new() -> Self {
        Self {
            col_names: Vec::new(),
            descriptions: Vec::new(),
            data: Vec::new(),
            num_cols: 0,
            num_rows: 0,
            order: Order::RowMajor,
        }
    }

    pub fn new_sized(
        num_cols: usize,
        num_rows: usize,
        col_names: Vec<String>,
    ) -> Result<Self, TableError> {
        if col_names.len() != num_cols {
            return Err(TableError::InvalidNumberOfColumnNames);
        }
        let data = vec![T::default(); num_cols * num_rows];
        Ok(Self {
            col_names,
            descriptions: Vec::from_iter("".repeat(num_cols).split(';').map(|s| s.to_string())),
            data,
            num_cols,
            num_rows,
            order: Order::default(),
        })
    }

    pub fn from_vec(
        data: Vec<T>,
        num_cols: usize,
        col_names: Vec<String>,
    ) -> Result<Self, TableError> {
        if data.len() % num_cols != 0 {
            return Err(TableError::InvalidNumberOfColumns);
        }
        if col_names.len() != num_cols {
            return Err(TableError::InvalidNumberOfColumnNames);
        }
        let length = data.len();
        Ok(Self {
            col_names,
            descriptions: Vec::from_iter("".repeat(num_cols).split(';').map(|s| s.to_string())),
            data,
            num_cols,
            num_rows: length / num_cols,
            order: Order::default(),
        })
    }

    pub fn from_vec_with_order(
        data: Vec<T>,
        num_cols: usize,
        col_names: Vec<String>,
        order: Order,
    ) -> Result<Self, TableError> {
        if data.len() % num_cols != 0 {
            return Err(TableError::InvalidNumberOfColumns);
        }
        let length = data.len();
        Ok(Self {
            col_names,
            descriptions: Vec::from_iter("".repeat(num_cols).split(';').map(|s| s.to_string())),
            data,
            num_cols,
            num_rows: length / num_cols,
            order,
        })
    }

    pub fn col(&self, col: usize) -> Option<&[T]> {
        if col >= self.num_cols {
            return None;
        }
        let mut col = col;
        if self.order == Order::ColumnMajor {
            col = col * self.num_rows;
        }
        Some(&self.data[col..col + self.num_rows])
    }

    pub fn col_mut(&mut self, col: usize) -> Option<&mut [T]> {
        if col >= self.num_cols {
            return None;
        }
        let mut col = col;
        if self.order == Order::ColumnMajor {
            col = col * self.num_rows;
        }
        Some(&mut self.data[col..col + self.num_rows])
    }

    pub fn col_name(&self, col: usize) -> Option<&str> {
        if col >= self.num_cols {
            return None;
        }
        self.col_names.get(col).map(|s| s.as_str())
    }

    pub fn col_name_mut(&mut self, col: usize) -> Option<&mut String> {
        if col >= self.num_cols {
            return None;
        }
        self.col_names.get_mut(col)
    }

    pub fn col_description(&self, col: usize) -> Option<&str> {
        if col >= self.num_cols {
            return None;
        }
        self.descriptions.get(col).map(|s| s.as_str())
    }

    pub fn col_description_mut(&mut self, col: usize) -> Option<&mut String> {
        if col >= self.num_cols {
            return None;
        }
        self.descriptions.get_mut(col)
    }

    pub fn col_names(&self) -> &[String] {
        &self.col_names
    }

    pub fn col_names_mut(&mut self) -> &mut [String] {
        &mut self.col_names
    }

    pub fn col_descriptions(&self) -> &[String] {
        &self.descriptions
    }

    pub fn col_descriptions_mut(&mut self) -> &mut [String] {
        &mut self.descriptions
    }

    pub fn row(&self, row: usize) -> Option<&[T]> {
        if row >= self.num_rows {
            return None;
        }
        let mut row = row;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        }
        Some(&self.data[row..row + self.num_cols])
    }

    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.num_rows {
            return None;
        }
        let mut row = row;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        }
        Some(&mut self.data[row..row + self.num_cols])
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn order(&self) -> Order {
        self.order
    }

    pub fn transpose(&mut self) {
        self.order.swap();
        std::mem::swap(&mut self.num_cols, &mut self.num_rows);
    }

    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    pub fn into_vec_with_names(self) -> (Vec<T>, Vec<String>) {
        (self.data, self.col_names)
    }

    pub fn col_by_name(&self, name: &str) -> Option<&[T]> {
        self.col_names
            .iter()
            .position(|s| s == name)
            .and_then(|i| self.col(i))
    }

    pub fn col_by_name_mut(&mut self, name: &str) -> Option<&mut [T]> {
        self.col_names
            .iter()
            .position(|s| s == name)
            .and_then(|i| self.col_mut(i))
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.num_rows || col >= self.num_cols {
            return None;
        }
        let mut row = row;
        let mut col = col;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        } else {
            col = col * self.num_rows;
        }
        Some(&self.data[row + col])
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row >= self.num_rows || col >= self.num_cols {
            return None;
        }
        let mut row = row;
        let mut col = col;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        } else {
            col = col * self.num_rows;
        }
        Some(&mut self.data[row + col])
    }

    pub unsafe fn get_unchecked(&self, row: usize, col: usize) -> &T {
        let mut row = row;
        let mut col = col;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        } else {
            col = col * self.num_rows;
        }
        &self.data[row + col]
    }

    pub unsafe fn get_unchecked_mut(&mut self, row: usize, col: usize) -> &mut T {
        let mut row = row;
        let mut col = col;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        } else {
            col = col * self.num_rows;
        }
        &mut self.data[row + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<T> {
        if row >= self.num_rows || col >= self.num_cols {
            return None;
        }
        let mut row = row;
        let mut col = col;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        } else {
            col = col * self.num_rows;
        }
        Some(std::mem::replace(&mut self.data[row + col], value))
    }

    pub fn set_col(&mut self, col: usize, col_data: &[T]) -> Option<Vec<T>>
    where
        T: Clone,
    {
        if col >= self.num_cols || col_data.len() != self.num_rows {
            return None;
        }
        let mut col = col;
        if self.order == Order::ColumnMajor {
            col = col * self.num_rows;
        }
        Some(
            std::mem::replace(
                &mut self.data[col..col + self.num_rows].to_vec(),
                col_data.to_vec(),
            )
            .into_iter()
            .collect(),
        )
    }

    pub fn set_row(&mut self, row: usize, row_data: &[T]) -> Option<Vec<T>>
    where
        T: Clone,
    {
        if row >= self.num_rows || row_data.len() != self.num_cols {
            return None;
        }
        let mut row = row;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        }
        Some(
            std::mem::replace(
                &mut self.data[row..row + self.num_cols].to_vec(),
                row_data.to_vec(),
            )
            .into_iter()
            .collect(),
        )
    }

    pub fn set_col_by_name(&mut self, name: &str, col_data: &[T]) -> Option<Vec<T>>
    where
        T: Clone,
    {
        self.col_names
            .iter()
            .position(|s| s == name)
            .and_then(|i| self.set_col(i, col_data))
    }

    pub fn set_col_name(&mut self, col: usize, name: String) -> Option<String> {
        if col >= self.num_cols {
            return None;
        }
        Some(std::mem::replace(&mut self.col_names[col], name))
    }

    pub fn set_col_description(&mut self, col: usize, description: String) -> Option<String> {
        if col >= self.num_cols {
            return None;
        }
        Some(std::mem::replace(&mut self.descriptions[col], description))
    }

    pub fn set_col_description_by_name(
        &mut self,
        name: &str,
        description: String,
    ) -> Option<String> {
        self.col_names
            .iter()
            .position(|s| s == name)
            .and_then(|i| self.set_col_description(i, description))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row >= self.num_rows {
            return None;
        }
        let mut row = row;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        }
        Some(self.data[row..row + self.num_cols].iter())
    }

    pub fn iter_row_mut(&mut self, row: usize) -> Option<impl Iterator<Item = &mut T>> {
        if row >= self.num_rows {
            return None;
        }
        let mut row = row;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        }
        Some(self.data[row..row + self.num_cols].iter_mut())
    }

    pub fn iter_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col >= self.num_cols {
            return None;
        }
        let mut col = col;
        if self.order == Order::ColumnMajor {
            col = col * self.num_rows;
        }
        Some(self.data[col..col + self.num_rows].iter())
    }

    pub fn iter_col_mut(&mut self, col: usize) -> Option<impl Iterator<Item = &mut T>> {
        if col >= self.num_cols {
            return None;
        }
        let mut col = col;
        if self.order == Order::ColumnMajor {
            col = col * self.num_rows;
        }
        Some(self.data[col..col + self.num_rows].iter_mut())
    }

    pub fn iter_col_by_name(&self, name: &str) -> Option<impl Iterator<Item = &T>> {
        self.col_names
            .iter()
            .position(|s| s == name)
            .and_then(|i| self.iter_col(i))
    }

    pub fn iter_col_by_name_mut(&mut self, name: &str) -> Option<impl Iterator<Item = &mut T>> {
        self.col_names
            .iter()
            .position(|s| s == name)
            .and_then(|i| self.iter_col_mut(i))
    }

    pub fn push_col(&mut self, name: String, col: Vec<T>) -> Result<(), TableError> {
        if col.len() != self.num_rows {
            return Err(TableError::InvalidNumberOfElements);
        }
        self.col_names.push(name);
        self.data.extend(col);
        self.num_cols += 1;
        self.descriptions.push("".to_string());
        Ok(())
    }

    pub fn push_row(&mut self, row: Vec<T>) -> Result<(), TableError> {
        if row.len() != self.num_cols {
            return Err(TableError::InvalidNumberOfElements);
        }
        self.data.extend(row);
        self.num_rows += 1;
        Ok(())
    }

    pub fn remove_col(&mut self, col: usize) -> Option<Vec<T>> {
        if col >= self.num_cols {
            return None;
        }
        let mut col = col;
        if self.order == Order::ColumnMajor {
            col = col * self.num_rows;
        }
        let out_col = self.data.drain(col..col + self.num_rows);
        self.col_names.remove(col);
        self.num_cols -= 1;
        self.descriptions.remove(col);
        Some(out_col.collect())
    }

    pub fn remove_row(&mut self, row: usize) -> Option<Vec<T>> {
        if row >= self.num_rows {
            return None;
        }
        let mut row = row;
        if self.order == Order::RowMajor {
            row = row * self.num_cols;
        }
        self.num_rows -= 1;
        Some(self.data.drain(row..row + self.num_cols).collect())
    }
}

