use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

use crate::astro::{
    brouwer_to_kozai, cartesian_to_keplerian, covariance_equinoctial_to_uvw, covariance_uvw_to_teme,
    ecr_to_efg, ecr_to_j2000, ecr_to_teme, efg_to_ecr, efg_to_j2000, efg_to_lla, efg_to_teme,
    equinoctial_to_keplerian, get_dll_info, get_earth_obstruction_angles, get_jpl_sun_and_moon_position,
    gst_ra_dec_to_az_el, gst_teme_to_lla, horizon_to_teme, j2000_to_ecr, j2000_to_efg, j2000_to_teme,
    keplerian_to_cartesian, keplerian_to_equinoctial, kozai_to_brouwer, lla_to_teme, llh_to_efg,
    mean_motion_to_sma, osculating_to_mean, point_is_sunlit, position_velocity_mu_to_equinoctial,
    position_velocity_to_equinoctial, set_jpl_ephemeris_file_path, sma_to_mean_motion, teme_to_ecr,
    teme_to_efg, teme_to_j2000, teme_to_topo, time_ra_dec_to_az_el, time_teme_to_lla, topo_meme_to_teme,
    topo_teme_to_meme,
};
use crate::DLL_VERSION;

#[pyclass]
pub struct AstroInterface {
    info: String,
}

#[pymethods]
impl AstroInterface {
    #[new]
    fn new() -> PyResult<Self> {
        let info = get_dll_info();
        if !info.contains(DLL_VERSION) {
            return Err(PyRuntimeError::new_err(format!(
                "Expected DLL {} inconsistent with {}",
                DLL_VERSION, info
            )));
        }
        Ok(AstroInterface { info })
    }

    #[getter]
    fn info(&self) -> PyResult<String> {
        Ok(self.info.clone())
    }

    fn keplerian_to_equinoctial(&self, kep: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(keplerian_to_equinoctial(&kep))
    }

    fn equinoctial_to_keplerian(&self, eqnx: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(equinoctial_to_keplerian(&eqnx))
    }

    fn keplerian_to_cartesian(&self, kep: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(keplerian_to_cartesian(&kep))
    }

    fn cartesian_to_keplerian(&self, posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(cartesian_to_keplerian(&posvel))
    }

    fn mean_motion_to_sma(&self, mean_motion: f64) -> PyResult<f64> {
        Ok(mean_motion_to_sma(mean_motion))
    }

    fn sma_to_mean_motion(&self, semi_major_axis: f64) -> PyResult<f64> {
        Ok(sma_to_mean_motion(semi_major_axis))
    }

    fn kozai_to_brouwer(&self, eccentricity: f64, inclination: f64, mean_motion: f64) -> PyResult<f64> {
        Ok(kozai_to_brouwer(eccentricity, inclination, mean_motion))
    }

    fn brouwer_to_kozai(&self, eccentricity: f64, inclination: f64, mean_motion: f64) -> PyResult<f64> {
        Ok(brouwer_to_kozai(eccentricity, inclination, mean_motion))
    }

    fn osculating_to_mean(&self, osc: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(osculating_to_mean(&osc))
    }

    fn position_velocity_to_equinoctial(&self, posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(position_velocity_to_equinoctial(&posvel))
    }

    fn position_velocity_mu_to_equinoctial(&self, posvel: [f64; 6], mu: f64) -> PyResult<[f64; 6]> {
        Ok(position_velocity_mu_to_equinoctial(&posvel, mu))
    }

    fn set_jpl_ephemeris_file_path(&self, file_path: String) -> PyResult<()> {
        set_jpl_ephemeris_file_path(&file_path);
        Ok(())
    }

    fn j2000_to_teme(&self, ds50_utc: f64, j2000_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(j2000_to_teme(ds50_utc, &j2000_posvel))
    }

    fn j2000_to_efg(&self, ds50_utc: f64, j2000_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(j2000_to_efg(ds50_utc, &j2000_posvel))
    }

    fn j2000_to_ecr(&self, ds50_utc: f64, j2000_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(j2000_to_ecr(ds50_utc, &j2000_posvel))
    }

    fn teme_to_j2000(&self, ds50_utc: f64, teme_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(teme_to_j2000(ds50_utc, &teme_posvel))
    }

    fn teme_to_efg(&self, ds50_utc: f64, teme_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(teme_to_efg(ds50_utc, &teme_posvel))
    }

    fn efg_to_ecr(&self, ds50_utc: f64, efg_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(efg_to_ecr(ds50_utc, &efg_posvel))
    }

    fn teme_to_ecr(&self, ds50_utc: f64, teme_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(teme_to_ecr(ds50_utc, &teme_posvel))
    }

    fn ecr_to_efg(&self, ds50_utc: f64, ecr_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(ecr_to_efg(ds50_utc, &ecr_posvel))
    }

    fn efg_to_teme(&self, ds50_utc: f64, efg_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(efg_to_teme(ds50_utc, &efg_posvel))
    }

    fn ecr_to_teme(&self, ds50_utc: f64, ecr_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(ecr_to_teme(ds50_utc, &ecr_posvel))
    }

    fn ecr_to_j2000(&self, ds50_utc: f64, ecr_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(ecr_to_j2000(ds50_utc, &ecr_posvel))
    }

    fn efg_to_j2000(&self, ds50_utc: f64, efg_posvel: [f64; 6]) -> PyResult<[f64; 6]> {
        Ok(efg_to_j2000(ds50_utc, &efg_posvel))
    }

    fn lla_to_teme(&self, ds50_utc: f64, pos_lla: [f64; 3]) -> PyResult<[f64; 3]> {
        Ok(lla_to_teme(ds50_utc, &pos_lla))
    }

    fn lla_to_efg(&self, pos_lla: [f64; 3]) -> PyResult<[f64; 3]> {
        Ok(llh_to_efg(&pos_lla))
    }

    fn topo_meme_to_teme(&self, yr_of_equinox: i32, ds50_utc: f64, ra: f64, dec: f64) -> PyResult<(f64, f64)> {
        Ok(topo_meme_to_teme(yr_of_equinox, ds50_utc, ra, dec))
    }

    fn topo_teme_to_meme(&self, yr_of_equinox: i32, ds50_utc: f64, ra: f64, dec: f64) -> PyResult<(f64, f64)> {
        Ok(topo_teme_to_meme(yr_of_equinox, ds50_utc, ra, dec))
    }

    fn covariance_equinoctial_to_uvw(
        &self,
        teme_posvel: [f64; 6],
        cov_eqnx: [[f64; 6]; 6],
    ) -> PyResult<[[f64; 6]; 6]> {
        Ok(covariance_equinoctial_to_uvw(&teme_posvel, &cov_eqnx))
    }

    fn covariance_uvw_to_teme(
        &self,
        teme_posvel: [f64; 6],
        cov_uvw: [[f64; 6]; 6],
    ) -> PyResult<[[f64; 6]; 6]> {
        Ok(covariance_uvw_to_teme(&teme_posvel, &cov_uvw))
    }

    fn gst_ra_dec_to_az_el(&self, gst: f64, lla: [f64; 3], ra: f64, dec: f64) -> PyResult<[f64; 2]> {
        Ok(gst_ra_dec_to_az_el(gst, &lla, ra, dec))
    }

    fn time_ra_dec_to_az_el(
        &self,
        ds50_utc: f64,
        lla: [f64; 3],
        ra: f64,
        dec: f64,
    ) -> PyResult<[f64; 2]> {
        Ok(time_ra_dec_to_az_el(ds50_utc, &lla, ra, dec))
    }

    fn horizon_to_teme(
        &self,
        lst: f64,
        lat: f64,
        sensor_teme: [f64; 3],
        xa_rae: [f64; 6],
    ) -> PyResult<[f64; 6]> {
        horizon_to_teme(lst, lat, &sensor_teme, &xa_rae).map_err(PyRuntimeError::new_err)
    }

    fn gst_teme_to_lla(&self, gst: f64, teme_pos: [f64; 3]) -> PyResult<[f64; 3]> {
        Ok(gst_teme_to_lla(gst, &teme_pos))
    }

    fn time_teme_to_lla(&self, ds50_utc: f64, teme_pos: [f64; 3]) -> PyResult<[f64; 3]> {
        Ok(time_teme_to_lla(ds50_utc, &teme_pos))
    }

    fn efg_to_lla(&self, efg_pos: [f64; 3]) -> PyResult<[f64; 3]> {
        efg_to_lla(&efg_pos).map_err(PyRuntimeError::new_err)
    }

    fn teme_to_topo(
        &self,
        lst: f64,
        lat: f64,
        sen_teme_pos: [f64; 3],
        sat_teme_posvel: [f64; 6],
    ) -> PyResult<[f64; 10]> {
        teme_to_topo(lst, lat, &sen_teme_pos, &sat_teme_posvel).map_err(PyRuntimeError::new_err)
    }

    fn get_jpl_sun_and_moon_position(&self, ds50utc: f64) -> PyResult<([f64; 3], [f64; 3])> {
        Ok(get_jpl_sun_and_moon_position(ds50utc))
    }

    fn point_is_sunlit(&self, ds50_tt: f64, teme_pos: [f64; 3]) -> PyResult<bool> {
        Ok(point_is_sunlit(ds50_tt, &teme_pos))
    }

    fn get_earth_obstruction_angles(
        &self,
        sat_teme_pos: [f64; 3],
        sensor_teme_pos: [f64; 3],
    ) -> PyResult<(f64, f64, f64)> {
        Ok(get_earth_obstruction_angles(&sat_teme_pos, &sensor_teme_pos))
    }
}

pub fn register_astro_interface(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<AstroInterface>()?;
    let class = parent_module.getattr("AstroInterface")?;
    class.setattr("XF_CONV_SGP42SGP", crate::astro::XF_CONV_SGP42SGP)?;
    Ok(())
}
