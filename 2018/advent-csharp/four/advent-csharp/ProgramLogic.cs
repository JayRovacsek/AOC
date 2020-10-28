using System;
using System.Collections.Generic;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;

namespace advent_csharp
{
    public class ProgramLogic
    {
        public IEnumerable<Log> LoadLogs(string filename) // 1518-08-17 00:00
        {
            var logs = new List<Log>();
            var line = string.Empty;
            var format = "yyyy-MM-dd H:mm";

            using (var sr = new StreamReader(filename))
            {
                while ((line = sr.ReadLine()) != null)
                {
                    var parts = line.Split(' ');
                    logs.Add(new Log
                    {
                        StartDate = DateTime.ParseExact($"{parts[0].Remove(0, 1)} {parts[1].Remove(parts[1].Length - 1, 1)}", format, CultureInfo.InvariantCulture),
                        GuardId = line.Contains('#') ? Convert.ToInt32(parts[3].Remove(0, 1)) : 0,
                        AsleepStart = line.Contains("asleep") ? true : false,
                        WakeStart = line.Contains("wakes") ? true : false,
                    });
                }
            }
            return logs;
        }

        public IEnumerable<Guard> GenerateGuardData(IEnumerable<Log> logs)
        {
            var guards = new List<Guard>();
            var ids = logs.Select(x => x.GuardId).Where(x => x != 0).Distinct();
            foreach (var id in ids)
                guards.Add(new Guard
                {
                    Id = id,
                    TimeAsleep = CalculateTimeAsleep(id, logs.ToList())
                });
            return guards;
        }

        public int CalculateTimeAsleep(int id, List<Log> logs)
        {
            var asleepTime = logs.Select(x => x).Where(x => x.GuardId == id && x.Asleep == true).Sum(x => x.EndDate.Subtract(x.StartDate).TotalMinutes);

            return Convert.ToInt32(asleepTime);
        }

        public Guard MostSleepy(IEnumerable<Guard> guards)
        {
            return guards.OrderByDescending(x => x.TimeAsleep).FirstOrDefault();
        }

        public int MostAsleepMinute(Guard guard, IEnumerable<Log> logs)
        {
            logs = AddGuardIds(logs.OrderBy(x => x.StartDate).ToList());
            logs = AddEndDate(logs.OrderBy(x => x.StartDate).ToList());
            logs = AddAsleepFlag(logs.ToList());
            var minutes = 60;
            var minuteCount = new Dictionary<int, int>();
            for (int i = 0; i < minutes; i++)
            {
                minuteCount[i] = 0;
            }

            foreach (var log in logs)
            {
                if (log.Asleep)
                {
                    var totalMinutes = log.StartDate != log.EndDate ? log.EndDate.Subtract(log.StartDate).TotalMinutes : 0;
                    var range = Enumerable.Range(log.StartDate.Minute, Convert.ToInt32(totalMinutes)).Select(x => x % 60);
                    foreach (var min in range)
                        minuteCount[min]++;
                }
            }
            return minuteCount.Select(x => x).Where(x => x.Value == minuteCount.Max(y => y.Value)).Select(z => z.Key).FirstOrDefault();
        }

        public IEnumerable<Log> AddGuardIds(List<Log> logs)
        {
            var guardId = 0;
            for (int i = 0; i < logs.Count(); i++)
            {
                logs[i].GuardId = logs[i].GuardId == 0 ? guardId : logs[i].GuardId;
                guardId = logs[i].GuardId;
            }

            return logs;
        }

        public IEnumerable<Log> AddEndDate(List<Log> logs)
        {
            var length = logs.Count() - 1;
            for (int i = 0; i < logs.Count(); i++)
            {
                logs[i].EndDate = i == length ? logs[i].StartDate : logs[i + 1].StartDate;
            }
            return logs;
        }

        public IEnumerable<Log> AddAsleepFlag(List<Log> logs)
        {
            var asleep = false;
            for (int i = 0; i < logs.Count(); i++)
            {
                asleep = logs[i].AsleepStart ? true : logs[i].WakeStart ? false : asleep;
                logs[i].Asleep = asleep;
            }
            return logs;
        }
    }
}
