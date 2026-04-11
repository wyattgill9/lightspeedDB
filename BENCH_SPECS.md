all from: https://tech.marksblogg.com/benchmarks.html


https://tech.marksblogg.com/billion-nyc-taxi-rides-redshift.html
```sql
CREATE FOREIGN TABLE trips (
    trip_id                 INTEGER,
    vendor_id               VARCHAR(3),

    pickup_datetime         TIMESTAMP,
    dropoff_datetime        TIMESTAMP,

    store_and_fwd_flag      VARCHAR(1),
    rate_code_id            SMALLINT,
    pickup_longitude        DOUBLE PRECISION,
    pickup_latitude         DOUBLE PRECISION,
    dropoff_longitude       DOUBLE PRECISION,
    dropoff_latitude        DOUBLE PRECISION,
    passenger_count         SMALLINT,
    trip_distance           DOUBLE PRECISION,
    fare_amount             DOUBLE PRECISION,
    extra                   DOUBLE PRECISION,
    mta_tax                 DOUBLE PRECISION,
    tip_amount              DOUBLE PRECISION,
    tolls_amount            DOUBLE PRECISION,
    ehail_fee               DOUBLE PRECISION,
    improvement_surcharge   DOUBLE PRECISION,
    total_amount            DOUBLE PRECISION,
    payment_type            VARCHAR(10),
    trip_type               SMALLINT,
    pickup                  VARCHAR(50),
    dropoff                 VARCHAR(50),

    cab_type                VARCHAR(10),

    precipitation           SMALLINT,
    snow_depth              SMALLINT,
    snowfall                SMALLINT,
    max_temperature         SMALLINT,
    min_temperature         SMALLINT,
    average_wind_speed      SMALLINT,

    pickup_nyct2010_gid     SMALLINT,
    pickup_ctlabel          VARCHAR(10),
    pickup_borocode         SMALLINT,
    pickup_boroname         VARCHAR(13),
    pickup_ct2010           VARCHAR(6),
    pickup_boroct2010       VARCHAR(7),
    pickup_cdeligibil       VARCHAR(1),
    pickup_ntacode          VARCHAR(4),
    pickup_ntaname          VARCHAR(56),
    pickup_puma             VARCHAR(4),

    dropoff_nyct2010_gid    SMALLINT,
    dropoff_ctlabel         VARCHAR(10),
    dropoff_borocode        SMALLINT,
    dropoff_boroname        VARCHAR(13),
    dropoff_ct2010          VARCHAR(6),
    dropoff_boroct2010      VARCHAR(7),
    dropoff_cdeligibil      VARCHAR(1),
    dropoff_ntacode         VARCHAR(4),
    dropoff_ntaname         VARCHAR(56),
    dropoff_puma            VARCHAR(4)
) SERVER gm_fdw_server OPTIONS (
    max_size '55682651',
    index '10:24')
  DISTRIBUTE BY ROUNDROBIN;
```

## QUERIES:
```sql
SELECT cab_type,
       count(*)
FROM trips
GROUP BY cab_type;
```

```sql
SELECT passenger_count,
       avg(total_amount)
FROM trips
GROUP BY passenger_count;
```

```sql
SELECT passenger_count,
       extract(year from pickup_datetime) AS pickup_year,
       count(*)
FROM trips
GROUP BY passenger_count,
         pickup_year;
```

```sql
SELECT passenger_count,
       extract(year from pickup_datetime) AS pickup_year,
       cast(trip_distance as int) AS distance,
       count(*) AS the_count
FROM trips
GROUP BY passenger_count,
         pickup_year,
         distance
ORDER BY pickup_year,
         the_count desc;
```
