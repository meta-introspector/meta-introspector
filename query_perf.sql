-- Query perf data from parquet

-- Summary by language
SELECT 
    language,
    COUNT(*) as total_samples,
    COUNT(DISTINCT ip) as unique_ips,
    ROUND(COUNT(DISTINCT ip) * 100.0 / COUNT(*), 1) as diversity_pct,
    COUNT(CASE WHEN dso NOT LIKE '%kernel%' THEN 1 END) as user_samples,
    ROUND(COUNT(CASE WHEN dso NOT LIKE '%kernel%' THEN 1 END) * 100.0 / COUNT(*), 1) as user_pct
FROM read_parquet('*_perf.parquet')
GROUP BY language
ORDER BY total_samples DESC;
