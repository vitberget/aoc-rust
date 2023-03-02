kernel void aoc_year2022_day4 (
    global int const* r1_lower,
    global int const* r1_upper,
    global int const* r2_lower,
    global int const* r2_upper,
    global int* result_1,
    global int* result_2
    )
{
    const size_t i = get_global_id(0);

    int r1l = r1_lower[i];
    int r1u = r1_upper[i];
    int r2l = r2_lower[i];
    int r2u = r2_upper[i];

    if(   r1l<=r2l && r1u>=r2u
       || r2l<=r1l && r2u>=r1u) {
        result_1[i] = 1;
    }

    if(   r2l >= r1l && r2l <= r1u
       || r2u >= r1l && r2u <= r1u
       || r1l >= r2l && r1l <= r2u) {
      result_2[i] = 1;
    } 
}