'use client';

import React, { useState, useEffect } from 'react';
import { 
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, AreaChart, Area 
} from 'recharts';
import { Zap, TrendingUp, Shield, Activity, ArrowUpRight, ArrowDownLeft, Plus } from 'lucide-react';

interface Trade {
  id: string;
  prosumer_address: string;
  consumer_address: string;
  amount_kwh: number;
  price_per_kwh: number;
  timestamp: string;
}

const mockChartData = [
  { name: '00:00', generation: 400, consumption: 240 },
  { name: '04:00', generation: 300, consumption: 139 },
  { name: '08:00', generation: 200, consumption: 980 },
  { name: '12:00', generation: 278, consumption: 390 },
  { name: '16:00', generation: 189, consumption: 480 },
  { name: '20:00', generation: 239, consumption: 380 },
  { name: '23:59', generation: 349, consumption: 430 },
];

export default function Dashboard() {
  const [trades, setTrades] = useState<Trade[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchTrades = async () => {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';
    try {
      const res = await fetch(`${apiUrl}/trades`);
      if (res.ok) {
        const data = await res.json();
        setTrades(data);
      }
    } catch (_err) {
      console.warn("Backend not available, using mock data");
      setTrades([
        { id: '1', prosumer_address: 'GB...123', consumer_address: 'GB...456', amount_kwh: 15.5, price_per_kwh: 0.12, timestamp: '2026-05-04T12:00:00Z' },
        { id: '2', prosumer_address: 'GB...789', consumer_address: 'GB...101', amount_kwh: 8.2, price_per_kwh: 0.15, timestamp: '2026-05-04T12:15:00Z' },
      ]);
    } finally {
      setLoading(false);
    }
  };

  const createTrade = async () => {
    const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';
    const newTrade = {
      id: crypto.randomUUID(),
      prosumer_address: "GB" + Math.random().toString(36).substring(7).toUpperCase(),
      consumer_address: "GB" + Math.random().toString(36).substring(7).toUpperCase(),
      amount_kwh: parseFloat((Math.random() * 20).toFixed(2)),
      price_per_kwh: parseFloat((Math.random() * 0.2).toFixed(2)),
    };

    try {
      const res = await fetch(`${apiUrl}/trades`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(newTrade),
      });
      if (res.ok) {
        fetchTrades();
      }
    } catch (_err) {
      setTrades([ { ...newTrade, timestamp: new Date().toISOString() }, ...trades ]);
    }
  };

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    fetchTrades();
  }, []);

  return (
    <div className="p-8 max-w-7xl mx-auto space-y-8">
      {/* Header */}
      <header className="flex flex-col md:flex-row justify-between items-start md:items-end gap-4">
        <div>
          <h1 className="text-4xl font-bold text-gradient">VoltChain Dashboard</h1>
          <p className="text-slate-400 mt-2">Empowering community energy trading on Stellar</p>
        </div>
        <div className="flex gap-4">
          <button 
            onClick={createTrade}
            className="glass px-6 py-3 rounded-2xl flex items-center gap-2 hover:bg-amber-500 hover:text-black transition-all group"
          >
            <Plus size={20} className="text-amber-500 group-hover:text-black" />
            <span className="font-bold">New Trade</span>
          </button>
          <div className="glass px-6 py-3 rounded-2xl flex items-center gap-4">
            <div className="text-right">
              <p className="text-xs text-slate-500 uppercase tracking-wider">Wallet Balance</p>
              <p className="text-xl font-mono font-bold text-amber-500">1,240.50 XLM</p>
            </div>
            <div className="w-10 h-10 rounded-full bg-amber-500/20 flex items-center justify-center">
              <Shield className="text-amber-500 w-5 h-5" />
            </div>
          </div>
        </div>
      </header>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
        {[
          { label: 'Current Generation', value: '4.2 kW', icon: Zap, color: 'text-amber-500' },
          { label: 'Total Traded', value: '1,280 kWh', icon: TrendingUp, color: 'text-emerald-500' },
          { label: 'Avg. Price', value: '0.13 XLM', icon: Activity, color: 'text-blue-500' },
          { label: 'Green Score', value: '98%', icon: Shield, color: 'text-purple-500' },
        ].map((stat, i) => (
          <div key={i} className="glass p-6 rounded-3xl hover:border-amber-500/30 transition-all duration-300">
            <div className={`w-12 h-12 rounded-2xl bg-white/5 flex items-center justify-center mb-4 ${stat.color}`}>
              <stat.icon size={24} />
            </div>
            <p className="text-slate-400 text-sm">{stat.label}</p>
            <p className="text-2xl font-bold mt-1">{stat.value}</p>
          </div>
        ))}
      </div>

      {/* Main Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 glass p-8 rounded-3xl h-[400px]">
          <h2 className="text-xl font-semibold mb-6 flex items-center gap-2">
            <Activity className="text-amber-500" /> Energy Activity (24h)
          </h2>
          <ResponsiveContainer width="100%" height="85%">
            <AreaChart data={mockChartData}>
              <defs>
                <linearGradient id="colorGen" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#f59e0b" stopOpacity={0.3}/>
                  <stop offset="95%" stopColor="#f59e0b" stopOpacity={0}/>
                </linearGradient>
                <linearGradient id="colorCons" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#3b82f6" stopOpacity={0.3}/>
                  <stop offset="95%" stopColor="#3b82f6" stopOpacity={0}/>
                </linearGradient>
              </defs>
              <CartesianGrid strokeDasharray="3 3" stroke="#ffffff10" vertical={false} />
              <XAxis dataKey="name" stroke="#ffffff40" fontSize={12} tickLine={false} axisLine={false} />
              <YAxis stroke="#ffffff40" fontSize={12} tickLine={false} axisLine={false} />
              <Tooltip 
                contentStyle={{ background: '#111', border: '1px solid #333', borderRadius: '12px' }}
                itemStyle={{ color: '#fff' }}
              />
              <Area type="monotone" dataKey="generation" stroke="#f59e0b" fillOpacity={1} fill="url(#colorGen)" strokeWidth={3} />
              <Area type="monotone" dataKey="consumption" stroke="#3b82f6" fillOpacity={1} fill="url(#colorCons)" strokeWidth={3} />
            </AreaChart>
          </ResponsiveContainer>
        </div>

        <div className="glass p-8 rounded-3xl overflow-hidden flex flex-col">
          <h2 className="text-xl font-semibold mb-6">Recent Trades</h2>
          <div className="space-y-6 overflow-y-auto flex-1 pr-2 custom-scrollbar">
            {trades.map((trade) => (
              <div key={trade.id} className="flex items-center justify-between group cursor-pointer animate-in fade-in slide-in-from-bottom-2 duration-300">
                <div className="flex items-center gap-4">
                  <div className="w-10 h-10 rounded-full flex items-center justify-center bg-amber-500/10 text-amber-500">
                    <ArrowUpRight size={20} />
                  </div>
                  <div>
                    <p className="font-medium">{trade.amount_kwh} kWh</p>
                    <p className="text-xs text-slate-500">{new Date(trade.timestamp).toLocaleTimeString()}</p>
                  </div>
                </div>
                <div className="text-right">
                  <p className="font-mono font-bold text-amber-500">{trade.price_per_kwh} XLM</p>
                  <p className="text-[10px] uppercase tracking-wider text-emerald-500">Completed</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
