using System.Diagnostics;
using System.Text.RegularExpressions;

class Program
{
    static void Main(string[] args)
    {
        string filePath = args[0];
        int resultP1 = 0;
        int resultP2 = 0;

        // look for mul(x,y)
        Regex rg = new Regex(@"mul\((\d{1,3}),(\d{1,3})\)|don\'t\(\)|do\(\)");

        bool enabled = true;

        Console.WriteLine("Advent of Code 2024 - Day 3");

        try
        {
            // Read and process the file line by line
            using (StreamReader sr = new StreamReader(filePath))
            {
                string line;
                while ((line = sr.ReadLine()) != null)
                {
                    MatchCollection matches = rg.Matches(line);

                    foreach (Match item in matches)
                    {
                        if (item.Value.StartsWith("don")){
                            enabled = false;
                            Console.WriteLine("ENABLED!");
                        } else if (item.Value.StartsWith("do")) {
                            enabled = true;
                            Console.WriteLine("DISABLEd!");
                        } else if (item.Value.StartsWith("mul")) {
                            int a = int.Parse(item.Groups[1].Value);
                            int b = int.Parse(item.Groups[2].Value);

                            Console.WriteLine($"Found {item.Value} ==> {a} * {b}");

                            resultP1 += a * b;

                            if (enabled) {
                                resultP2 += a * b;
                            }
                        }
                        
                    }
                }
            }

            Console.WriteLine($"Result of P1: {resultP1}");
            Console.WriteLine($"Result of P2: {resultP2}");
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
}