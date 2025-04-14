pub mod solution{
    use std::fmt;
    use std::ops;
    use std::default::Default;

    pub struct ComplexNumber{
        pub real: f64,
        pub imag: f64
    }
    #[derive(Debug, PartialEq)]
    pub enum ComplexNumberError{
        ImaginaryNotZero,
    }
    impl ComplexNumber{
        pub fn new(real: f64, imag: f64) -> ComplexNumber{
            ComplexNumber {
                real,
                imag
            }
        }
        pub fn real(&self) -> f64{
            self.real
        }
        pub fn imag(&self) -> f64{
            self.imag
        }
        pub fn from_real(num: f64) -> ComplexNumber{
           let a = Self::new(num, 0.0);
            return a
        }
        pub fn to_tuple(&self) -> (f64, f64){
            (self.real, self.imag)
        }
    }
    impl fmt::Display for ComplexNumber{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }
    impl ops::Add for ComplexNumber{
        type Output = ComplexNumber;

        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.real + rhs.real, self.imag + rhs.imag)
        }
    }
    impl ops::Add<f64> for ComplexNumber {
        type Output = ComplexNumber;

        fn add(self, rhs: f64) -> Self::Output {
            Self::new(
                self.real + rhs,  // Somma rhs alla parte reale
                self.imag,        // La parte immaginaria rimane invariata
            )
        }
    }
    impl ops::Add<&ComplexNumber> for ComplexNumber{
        type Output = ComplexNumber;
        fn add(self, rhs:&ComplexNumber) -> Self::Output {
            Self::new(
                self.real + rhs.real,
                self.imag + rhs.imag,
            )
        }
    }
    impl ops::Add<&ComplexNumber> for &ComplexNumber{
        type Output = ComplexNumber;
        fn add(self, rhs: &ComplexNumber) -> Self::Output{
            ComplexNumber::new(
                self.real + rhs.real,
                self.imag + rhs.imag,
            )
        }
    }
    impl ops::AddAssign for ComplexNumber{
        fn add_assign(&mut self, rhs: Self){
            self.real+=rhs.real;
            self.imag+=rhs.imag;
        }
    }
    impl Default for ComplexNumber{
        fn default() -> Self{
            ComplexNumber::new(0.0, 0.0)
        }
    }
    impl Into<f64> for ComplexNumber{
        fn into(self) -> f64{
            if(self.imag != 0.0){
                panic!("Cannot convert ComplexNumber with non-zero part into f64");
            }
            self.real
        }
    }

    /*impl TryFrom<ComplexNumber> for f64 {
        type Error = ComplexNumberError;

        fn try_from(value: ComplexNumber) -> Result<Self, Self::Error> {
            if value.imag == 0.0 {
                Ok(value.real)
            } else {
                Err(ComplexNumberError::ImaginaryNotZero)
            }
        }
    }*/

}