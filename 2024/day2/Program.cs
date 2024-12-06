﻿class Program
{
    static void Main(string[] args)
    {
        string filePath = "input.txt";
        int safeReports = 0;
        int dampenedReports = 0;

        Console.WriteLine("Advent of Code 2024 - Day 2");

        try
        {
            // Read and process the file line by line
            using (StreamReader sr = new StreamReader(filePath))
            {
                string line;
                while ((line = sr.ReadLine()) != null)
                {
                    List<int> values = ProcessLine(line);

                    if (IsSafe(values)) {
                        safeReports++;
                    } else if (Dampener(values)){
                        dampenedReports++;
                    }
                }
            }

            Console.WriteLine($"Part I Answer; {safeReports}");
            safeReports += dampenedReports;
            Console.WriteLine($"Part II Answer; {safeReports}");
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

    static bool Dampener(List<int> values)
    {
        bool isSafe = false;

        for (int i = 0; i < values.Count; i++) {
            List<int> modifiedList = new List<int>(values); 
            modifiedList.RemoveAt(i);

            if (IsSafe(modifiedList)) {
                return true;
            }
        }
        return isSafe;
    }

    static bool IsSafe(List<int> values)
    {
        bool isSafe = true;
        bool isAsc = true;
        bool isDesc = true;

        for (int i = 0; i < values.Count - 1; i++) {
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

    static List<int> ProcessLine(string line)
    {
        return line.Split(" ").Select(int.Parse).ToList();
    }
}
