class Program
{
    static void Main(string[] args)
    {
        string filePath = args[0];

        Console.WriteLine("Advent of Code 2024 - Day 5");

        var rules = new List<int[]>();
        var updates = new List<int[]>();
        
        bool isNumbersSection = false;
        
        // Read file structure
        foreach (var line in File.ReadLines(filePath))
        {
            if (string.IsNullOrWhiteSpace(line))
            {
                isNumbersSection = true;
                continue;
            }
            
            if (!isNumbersSection)
            {
                var parts = line.Split('|');
                if (parts.Length == 2 && int.TryParse(parts[0], out int part1) && int.TryParse(parts[1], out int part2))
                {
                    rules.Add(new int[] { part1, part2 });
                }
            }
            else
            {
                var parts = line.Split(',');
                var numberList = new List<int>();
                foreach (var part in parts)
                {
                    if (int.TryParse(part.Trim(), out int number))
                    {
                        numberList.Add(number);
                    }
                }
                updates.Add(numberList.ToArray());
            }
        }
        
        // Part I
        int partIAnswer = 0;

        // Cada Update
        foreach (var update in updates)
        {
            bool valid_update = true;

            foreach (var rule in rules)
            {
                var left  = Array.IndexOf(update, rule[0]);
                var right  = Array.IndexOf(update, rule[1]);

                if (left < 0 || right < 0) {
                    continue;
                }

                if (left > right) {
                    valid_update = false;
                }
            }
            if (valid_update) {
                partIAnswer += update[update.Length / 2];
            }
        }

        Console.WriteLine($"First part is: {partIAnswer}");
    }
}
