﻿class Program
{
    static void Main(string[] args)
    {
        string filePath = "input.txt";
        int safeReports = 0;

        Console.WriteLine("Advent of Code 2024 - Day 2");

        try
        {
            // Read and process the file line by line
            using (StreamReader sr = new StreamReader(filePath))
            {
                string line;
                while ((line = sr.ReadLine()) != null)
                {
                    int[] values = ProcessLine(line);

                    if (IsSafe(values)) {
                        safeReports++;
                    }
                }
            }

            Console.WriteLine($"Part I Answer; {safeReports}");
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

    static bool IsSafe(int[] values)
    {
        bool isSafe = true;
        bool isAsc = true;
        bool isDesc = true;

        for (int i = 0; i < values.Length - 1; i++) {
            int diff = values[i + 1] - values[i];

            if (diff > 0) {
                isDesc = false;
            }

            if (diff < 0) {
                isAsc = false;
            }

            diff = Math.Abs(diff);

            if ((diff < 1) || (diff > 3)) {
                isSafe = false;
                break;
            }

        }

        return isSafe && (isAsc || isDesc);
    }

    static int[] ProcessLine(string line)
    {
        return line.Split(" ").Select(int.Parse).ToArray();
    }
}
