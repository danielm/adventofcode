using System.Diagnostics;
using System.Text.RegularExpressions;

class Program
{
    static void Main(string[] args)
    {
        string filePath = args[0];
        int resultP1 = 0;

        Console.WriteLine("Advent of Code 2024 - Day 3");

        try
        {
            // Read and process the file line by line
            using (StreamReader sr = new StreamReader(filePath))
            {
                string line;
                while ((line = sr.ReadLine()) != null)
                {
                    resultP1 += ProcessLine(line);
                }
            }

            Console.WriteLine($"Result of P1: {resultP1}");
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

    static int ProcessLine(string line)
    {
        // look for mul(x,y)
        Regex rg = new Regex(@"mul\((\d{1,3}),(\d{1,3})\)");
        
        int lineResult = 0;

        MatchCollection matches = rg.Matches(line);

        foreach (Match item in matches)
        {
            int a = int.Parse(item.Groups[1].Value);
            int b = int.Parse(item.Groups[2].Value);

            Console.WriteLine($"Found {item.Value} ==> {a} * {b}");

            lineResult += a * b;
        }
        return lineResult;
    }
}