class Program
{
    const string MagicWord = "XMAS";

    static void Main(string[] args)
    {
        string filePath = args[0];

        Console.WriteLine("Advent of Code 2024 - Day 4");

        AOCMatrix matrix = new AOCMatrix(File.ReadAllLines(filePath));

        int foundWords = 0;

        for (int row = 0; row < matrix.RowCount; row++) {
            for (int col = 0; col < matrix.ColCount; col++) {
                char c = matrix[row, col];

                // Look for our first character--> X
                if (c != MagicWord[0]) {
                    continue;
                }

                // Considering we are at C, need to check all neighbors
                // A way to do it is from {row-1}{col-1}
                //[?][?][?] [?]
                //[?][X][?] [?]
                //[?][?][M] [?]

                //[?][?][?] [A]
                //
                // - Validate these are valid coords
                // - Validate characters at this positions are the ones we want
                // 'bx' and 'b'y' are basically our 'directions' to check

                for (int bx = -1; bx <= 1; bx++){
                    for (int by = -1; by <= 1; by++){
                        if (bx == 0 && by == 0) {
                            // This is literally  => 'X' (same as 'c' var)
                            continue;
                        }

                        // Checks a Word at our current position, in the direction (bx,by)
                        if (matrix.CheckWord(MagicWord, row, col, bx, by)) {
                            foundWords++;
                        }
                    }
                }
            }
        }

        Console.WriteLine($"Part 1 Answer: {foundWords}");
    }
}

class AOCMatrix
{
    private string[] _matrix;

    public int RowCount { get; }
    public int ColCount { get; }

    public AOCMatrix(string[] matrix)
    {
        _matrix = matrix;
        RowCount = matrix.Length;
        ColCount = matrix[0].Length;
    }

    public char this[int row, int col]
    {
        get { return _matrix[row][col]; }
    }

    public bool CheckWord(
        string needle,
        int row, int col, int bx, int by
    )
    {
        bool foundWord = true;

        // We could Start at 1 since we dont need to check the first char, but for understanding reasons
        for (int i = 0; i < needle.Length; i++) {
            // These two *1 --> current row + direction * next character to check since whatever
            //                  direction we checking should exponentially incesing  
            int checkRow = row + bx * i;//
            int checkCol = col + by * i;//

            if (!ValidCoords(checkRow, checkCol) ||
                    _matrix[checkRow][checkCol] != needle[i]) {
                foundWord = false;
                break;
            }
        }
        return foundWord;
    }

    public bool ValidCoords(int row, int col)
    {
        return row >= 0 && row < RowCount && col >= 0 && col < ColCount;
    }
}
