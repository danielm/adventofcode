class Program
{
    static void Main(string[] args)
    {
        string filePath = args[0];

        Console.WriteLine("Advent of Code 2024 - Day 6");

        string[] matrix = File.ReadAllLines(args[0]);
        int matrix_w = matrix[0].Length;
        int matrix_h = matrix.Length;

        List<(int,int)> directions = new List<(int, int)>{
            // row,col .. y,x
            // ^ = previous row, same col: (-1, 0)
            // > = same row, next col: (0, 1)
            // ▼ = next row, same col: (1, 0)
            // < = same col, previous col: (0, -1)
            (-1,0), (0,1), (1,0), (0,-1)
        };

        (int,int) guard = ( -1, -1);

        // Find starting position coords, save it
        // Also Replace it with a dot so we have a clean map when we move
        for (int row = 0; row < matrix_h; row++)
        {
            var col = matrix[row].IndexOf('^');

            if (col >= 0) {
                guard = (row, col);

                matrix[row] = matrix[row].Replace('^', '.');
                break;
            }
        }

        Console.WriteLine($"Guard found at: {guard.Item1},{guard.Item2}");

        // index of the directions list 
        // our starting position is directions[0] (caret pointing up)
        int currentDir = 0;

        List<(int, int)> visited = new List<(int, int)>();

        while(true)
        {
            if (!visited.Contains(guard)) {
                visited.Add(guard);
            }
            // Our next position is: 
            // - current postion col + direction col
            // - current position row + direction row
            int newRow = guard.Item1 + directions[currentDir].Item1;
            int newCol = guard.Item2 + directions[currentDir].Item2;

            // checking if outside our matrix? ==> its over
            if (newRow >= matrix_h || newCol >= matrix_w || newRow < 0 || newCol < 0) {
                break;
            }

            // its a dot? ==> move there
            if (matrix[newRow][newCol] == '.') {
                guard = (newRow, newCol);
                continue;
            }

            // found non dot character? ==> rotate 90degrees right
            // (rotate means change current direction to our next)
            // the module % operator, is used to keep rotating (wrap arround)
            // our direction list on eche requested rotation: 
            // dir: 0, 1, 2, 3 .. back to 0, 1, 2, 3
            currentDir = ( currentDir + 1 ) % directions.Count;
        }

        Console.WriteLine($"We have visited: {visited.Count}");
    }
}
