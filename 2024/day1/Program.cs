class Program
{
    static void Main(string[] args)
    {
        string filePath = "input.txt";

        List<int> col1 = new List<int>();
        List<int> col2 = new List<int>();

        int totalDistance = 0;
        int similarityScore = 0;

        Console.WriteLine("Advent of Code 2024 - Day 1");

        try
        {
            // Read and process the file line by line
            using (StreamReader sr = new StreamReader(filePath))
            {
                string line;
                while ((line = sr.ReadLine()) != null)
                {
                    int[] values = ProcessLine(line);

                    col1.Add(values[0]);
                    col2.Add(values[1]);
                }
            }

            // Part I
            col1.Sort();
            col2.Sort();

            for (int i = 0; i < col1.Count; i++) {
                totalDistance += Math.Abs(col2[i] - col1[i]);
            }

            Console.WriteLine($"Result of Part I: {totalDistance}");

            // Part II
            col1.ForEach(number => similarityScore += CalculateScoreFor(number, col2));

            Console.WriteLine($"Result of Part II: {similarityScore}");
        }
        catch (FileNotFoundException)
        {
            Console.WriteLine("File not found: " + filePath);
        }
        catch (Exception e)
        {
            Console.WriteLine("An error occurred: " + e.Message);
        }
    }

    static int CalculateScoreFor(int target, List<int> list)
    {
        return target * list.Count(number => number == target);
    }

    static int[] ProcessLine(string line)
    {
        string[] values = line.Split("   ");

        return [int.Parse(values[0]), int.Parse(values[1])];
    }
}
