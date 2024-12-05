class Program
{
    static void Main(string[] args)
    {
        string filePath = "input.txt";

        List<int> col1 = new List<int>();
        List<int> col2 = new List<int>();

        int totalDistance = 0;

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

            //Console.Write($"Col 1 lenght: {col1.Count} - Col 2 lenght: {col2.Count}");

            col1.Sort();
            col2.Sort();

            for (int i = 0; i < col1.Count; i++) {
                totalDistance += Math.Abs(col2[i] - col1[i]);
            }

            Console.Write($"Result is {totalDistance}");
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

    static int[] ProcessLine(string line)
    {
        string[] values = line.Split("   ");

        return [int.Parse(values[0]), int.Parse(values[1])];
    }
}
