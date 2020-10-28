using System;
using System.Collections.Generic;
using System.Text;

namespace advent_csharp
{
    public class Log
    {
        public int GuardId { get; set; }
        public DateTime StartDate { get; set; }
        public DateTime EndDate { get; set; }
        public bool AsleepStart { get; set; }
        public bool WakeStart { get; set; }
        public bool Asleep { get; set; }
    }
}
