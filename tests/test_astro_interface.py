import math

import pytest

from pysaal import AstroInterface, MainInterface, TimeInterface


def hour_min_sec_to_deg(hr: float, mn: float, sc: float) -> float:
    return (hr / 24.0 + mn / (24.0 * 60.0) + sc / (24.0 * 60.0 * 60.0)) * 360.0


def deg_min_sec_to_deg(deg: float, mn: float, sc: float) -> float:
    if deg < 0.0:
        return deg - mn / 60.0 - sc / 3600.0
    return deg + mn / 60.0 + sc / 3600.0


def test_get_dll_info() -> None:
    interface = AstroInterface()
    assert MainInterface.DLL_VERSION in interface.info


def test_keplerian_to_equinoctial() -> None:
    interface = AstroInterface()
    kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612]
    eqnx = interface.keplerian_to_equinoctial(kep)

    assert eqnx[0] == pytest.approx(0.005756008409, abs=1.0e-12)
    assert eqnx[1] == pytest.approx(0.002453246053, abs=1.0e-12)
    assert eqnx[2] == pytest.approx(0.130405060328, abs=1.0e-12)
    assert eqnx[3] == pytest.approx(-0.503224317374, abs=1.0e-12)
    assert eqnx[4] == pytest.approx(617.8480000000, abs=1.0e-12)
    assert eqnx[5] == pytest.approx(2.005848298418, abs=1.0e-12)


def test_equinoctial_to_keplerian() -> None:
    interface = AstroInterface()
    eqnx = [
        0.005756008409,
        0.002453246053,
        0.130405060328,
        -0.503224317374,
        617.8480000,
        2.005848298418,
    ]
    kep = interface.equinoctial_to_keplerian(eqnx)

    assert kep[0] == pytest.approx(26558.4820, abs=1.0e-4)
    assert kep[1] == pytest.approx(0.0062570000, abs=1.0e-7)
    assert kep[2] == pytest.approx(54.9350000, abs=1.0e-7)
    assert kep[3] == pytest.approx(234.7640000, abs=1.0e-7)
    assert kep[4] == pytest.approx(165.4720000, abs=1.0e-7)
    assert kep[5] == pytest.approx(217.6120000, abs=1.0e-7)


def test_keplerian_to_cartesian() -> None:
    interface = AstroInterface()
    kep = [26558.482, 0.006257, 54.935, 234.764, 165.472, 217.612]
    posvel = interface.keplerian_to_cartesian(kep)

    assert posvel[0] == pytest.approx(-3032.21272487, abs=1.0e-7)
    assert posvel[1] == pytest.approx(-15025.7763831, abs=1.0e-7)
    assert posvel[2] == pytest.approx(21806.4954366, abs=1.0e-7)
    assert posvel[3] == pytest.approx(3.754350020203, abs=1.0e-7)
    assert posvel[4] == pytest.approx(-0.889562019024, abs=1.0e-7)
    assert posvel[5] == pytest.approx(-0.114933710268, abs=1.0e-7)


def test_cartesian_to_keplerian() -> None:
    interface = AstroInterface()
    posvel = [
        -3032.21272487,
        -15025.7763831,
        21806.4954366,
        3.7543500202,
        -0.889562019026,
        -0.114933710268,
    ]
    kep = interface.cartesian_to_keplerian(posvel)

    assert kep[0] == pytest.approx(26558.4820, abs=1.0e-4)
    assert kep[1] == pytest.approx(0.0062570000, abs=1.0e-7)
    assert kep[2] == pytest.approx(54.9350000, abs=1.0e-7)
    assert kep[3] == pytest.approx(234.7640000, abs=1.0e-7)
    assert kep[4] == pytest.approx(165.4720000, abs=1.0e-7)
    assert kep[5] == pytest.approx(217.6120000, abs=1.0e-7)


def test_mean_motion_conversions() -> None:
    interface = AstroInterface()
    mean_motion = 1.0027382962
    semi_major_axis = interface.mean_motion_to_sma(mean_motion)
    assert semi_major_axis == pytest.approx(42164.171420, abs=1.0e-6)

    mean_motion_back = interface.sma_to_mean_motion(42164.17142)
    assert mean_motion_back == pytest.approx(mean_motion, abs=1.0e-7)


def test_kozai_brouwer_conversions() -> None:
    interface = AstroInterface()
    kozai = 14.2024103100000
    brouwer = 14.2107268431215
    ecc = 1.112700000000000e-02
    inc = 99.4371000000000

    to_brouwer = interface.kozai_to_brouwer(ecc, inc, kozai)
    assert to_brouwer == pytest.approx(brouwer, abs=1.0e-7)

    to_kozai = interface.brouwer_to_kozai(ecc, inc, brouwer)
    assert to_kozai == pytest.approx(kozai, abs=1.0e-7)


def test_osculating_to_mean() -> None:
    interface = AstroInterface()
    osc = [7200.0, 0.006257, 54.935, 234.764, 165.472, 217.612]
    mean = interface.osculating_to_mean(osc)

    assert mean[0] == pytest.approx(7206.06814087, abs=1.0e-7)
    assert mean[1] == pytest.approx(0.00646986051778, abs=1.0e-7)
    assert mean[2] == pytest.approx(54.9518948032, abs=1.0e-7)
    assert mean[3] == pytest.approx(232.98566416, abs=1.0e-7)
    assert mean[4] == pytest.approx(165.473342418, abs=1.0e-7)
    assert mean[5] == pytest.approx(219.392452222, abs=1.0e-7)


def test_time_teme_to_lla_without_tcon() -> None:
    main = MainInterface()
    ti = TimeInterface()
    interface = AstroInterface()
    ti.clear_constants()
    try:
        ds50_utc = 17687.91562858796
        xyz = [6524.834, 6862.875, 6448.296]
        llh = interface.time_teme_to_lla(ds50_utc, xyz)

        assert llh[0] == pytest.approx(34.3524936102065, abs=1.0e-9)
        assert llh[1] == pytest.approx(183.6827264765011, abs=1.0e-9)
        assert llh[2] == pytest.approx(5085.220665718614, abs=1.0e-9)
    finally:
        main.initialize_time_constants()


def test_lla_to_teme_without_tcon() -> None:
    main = MainInterface()
    ti = TimeInterface()
    interface = AstroInterface()
    ti.clear_constants()
    try:
        ds50_utc = 17687.91562858796
        llh = [34.3524936102065, 183.6827264765011, 5085.220665718614]
        xyz = interface.lla_to_teme(ds50_utc, llh)

        assert xyz[0] == pytest.approx(6524.834045160657, abs=1.0e-9)
        assert xyz[1] == pytest.approx(6862.875047500358, abs=1.0e-9)
        assert xyz[2] == pytest.approx(6448.295904107691, abs=1.0e-9)
    finally:
        main.initialize_time_constants()


def test_ra_dec_to_az_el_without_tcon() -> None:
    main = MainInterface()
    ti = TimeInterface()
    interface = AstroInterface()
    ti.clear_constants()
    try:
        ds50_utc = ti.dtg_to_ds50("13/217 0714 13.657")
        theta_g = ti.get_fk5_greenwich_angle(ti.utc_to_ut1(ds50_utc))
        ra = hour_min_sec_to_deg(21.0, 45.0, 19.003)
        dec = deg_min_sec_to_deg(-3.0, 17.0, 54.51)
        lla = [20.71126, 203.7394, 0.0]

        az_el = interface.gst_ra_dec_to_az_el(theta_g, lla, ra, dec)
        assert az_el[0] == pytest.approx(104.90532853088844, abs=1.0e-10)
        assert az_el[1] == pytest.approx(26.497513882129642, abs=1.0e-10)

        az_el_time = interface.time_ra_dec_to_az_el(ds50_utc, lla, ra, dec)
        assert az_el_time[0] == pytest.approx(az_el[0], abs=1.0e-10)
        assert az_el_time[1] == pytest.approx(az_el[1], abs=1.0e-10)
    finally:
        main.initialize_time_constants()


def test_horizon_to_teme() -> None:
    interface = AstroInterface()
    lst = 4.01991574771239
    lat = 54.0
    xa_rae = [
        0.430460160479830 * 6378.135,
        311.60356010055284,
        0.0003630520892354455,
        -2.77471740320679,
        -0.143557569934800,
        2.461934326381368e-2,
    ]
    sensor_teme = [-2398.87840986937, -2891.94814468770, 5136.98500000000]
    posvel = interface.horizon_to_teme(lst, lat, sensor_teme, xa_rae)

    assert posvel[0] == pytest.approx(-3037.43093289, abs=1.0e-7)
    assert posvel[1] == pytest.approx(-446.126832813, abs=1.0e-7)
    assert posvel[2] == pytest.approx(6208.50743365, abs=1.0e-7)
    assert posvel[3] == pytest.approx(-5.937185805, abs=1.0e-7)
    assert posvel[4] == pytest.approx(-3.51389427125, abs=1.0e-7)
    assert posvel[5] == pytest.approx(-3.15199314614, abs=1.0e-7)


def test_teme_to_topo() -> None:
    interface = AstroInterface()
    lst = 4.01991574771239
    lat = 0.942477796076938 * 180.0 / math.pi
    sen_teme_pos = [-2398.87840986937, -2891.94814468770, 5136.98500000000]
    sat_teme_posvel = [
        -3037.43125693340,
        -446.126917413657,
        6208.50743364866,
        -5.93718561230045,
        -3.51389500931854,
        -3.15199346948741,
    ]
    topo = interface.teme_to_topo(lst, lat, sen_teme_pos, sat_teme_posvel)

    assert topo[0] == pytest.approx(104.63211485, abs=1.0e-4)
    assert topo[1] == pytest.approx(22.9718279282, abs=1.0e-4)
    assert topo[2] == pytest.approx(311.60356010055284, abs=1.0e-4)
    assert topo[3] == pytest.approx(0.0003630520892354455, abs=1.0e-4)
    assert topo[4] == pytest.approx(0.430460160479830 * 6378.135, abs=1.0e-4)
    assert topo[5] == pytest.approx(0.15395211773785697, abs=1.0e-7)
    assert topo[6] == pytest.approx(-0.046898266550268346, abs=1.0e-7)
    assert topo[7] == pytest.approx(-0.143557569934800, abs=1.0e-7)
    assert topo[8] == pytest.approx(2.461934326381368e-2, abs=1.0e-7)
    assert topo[9] == pytest.approx(-2.77471740320679, abs=1.0e-7)


def test_point_is_sunlit() -> None:
    interface = AstroInterface()
    ds50_tt = 18989.0
    pt = [5032.21272487, 2025.7763831, 3106.4954366]

    assert interface.point_is_sunlit(ds50_tt, pt) is False
    pt[1] = -2025.7763831
    assert interface.point_is_sunlit(ds50_tt, pt) is True
