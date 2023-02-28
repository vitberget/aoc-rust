kernel void aoc_year2022_day4 (
    global int const* r1_lower,
    global int const* r1_upper,
    global int const* r2_lower,
    global int const* r2_upper,
    global int* result
    )
{
    const size_t i = get_global_id(0);

    //int r = 0;

    if(r1_lower[i]<=r2_lower[i] && r1_upper[i]>=r2_upper[i]) {
        result[i] = 1;
      //  r = 1;
    } else if(r2_lower[i]<=r1_lower[i] && r2_upper[i]>=r1_upper[i]) {
        result[i] = 1;
      //  r = 1;
    }

    //printf("hello %i, %i-%i,%i-%i - %i\n", (i+1), r1_lower[i], r1_upper[i], r2_lower[i], r2_upper[i], r);
}