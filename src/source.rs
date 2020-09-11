use std::rc::Rc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Source
{
    start_line: usize,
    end_line:   usize,
    start:      usize,
    end:        usize,
    all_text:   Rc<String>,
    all_lines:  Rc<Vec<String>>,
}

impl Source
{
    pub fn new(
        start_line: usize,
        end_line: usize,
        start: usize,
        end: usize,
        all_text: String,
    ) -> Source
    {
        Source {
            start_line,
            end_line,
            start,
            end,
            all_text: Rc::new(all_text.clone()),
            all_lines: Rc::new(all_text.split('\n').map(String::from).collect()),
        }
    }

    pub fn is_empty(&self) -> bool
    {
        self.start == self.end
    }

    pub fn merge(sources: Vec<Source>) -> Source
    {
        if sources.is_empty()
        {
            return Source::empty();
        }

        let mut min_start_line = usize::MAX;
        let mut max_end_line = 0;

        let mut min_start = usize::MAX;
        let mut max_end = 0;

        let mut all_text = Rc::new(String::new());
        let mut all_lines = Rc::new(Vec::new());

        let mut all_are_empty = true;

        for source in sources
        {
            // Ignore empty sources
            if source.is_empty()
            {
                continue;
            }

            all_are_empty = false;

            if source.start_line < min_start_line
            {
                min_start_line = source.start_line;
            }

            if source.end_line > max_end_line
            {
                max_end_line = source.end_line;
            }

            if source.start < min_start
            {
                min_start = source.start;
            }

            if source.end > max_end
            {
                max_end = source.end
            }

            all_text = source.all_text;
            all_lines = source.all_lines;
        }

        if all_are_empty
        {
            Source::empty()
        }
        else
        {
            Source {
                start_line: min_start_line,
                end_line: max_end_line,
                start: min_start,
                end: max_end,
                all_text,
                all_lines,
            }
        }
    }

    pub fn extend(&mut self, length: usize)
    {
        self.end += length;
    }

    pub fn get_range(&self, start_line: usize, end_line: usize, start: usize, end: usize)
        -> Source
    {
        Source {
            start_line: start_line + self.start_line,
            end_line:   end_line + self.start_line,
            start:      start + self.start,
            end:        end + self.start,
            all_text:   Rc::clone(&self.all_text),
            all_lines:  Rc::clone(&self.all_lines),
        }
    }

    pub fn empty() -> Source
    {
        Source {
            start_line: 0,
            end_line:   0,
            start:      0,
            end:        0,
            all_text:   Rc::new(String::new()),
            all_lines:  Rc::new(Vec::new()),
        }
    }

    pub fn get_start_line(&self) -> usize
    {
        return self.start_line;
    }

    pub fn get_end_line(&self) -> usize
    {
        return self.end_line;
    }

    pub fn get_text(&self) -> &str
    {
        &self.all_text[self.start..self.end]
    }

    pub fn get_all_lines(&self) -> &Vec<String>
    {
        self.all_lines.as_ref()
    }
}
